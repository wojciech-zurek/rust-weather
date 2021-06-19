use serde::Deserialize;
use chrono::{Utc, DateTime};
use chrono::serde::ts_seconds;
use std::collections::{HashMap, BTreeSet};

const DIRECTIONS: &'static [&str] = &["N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW", "N"];

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub main: Main,
    pub name: String,
    pub sys: Sys,
    pub weather: Vec<Weather>,
    pub wind: Wind,
}

#[derive(Debug, Deserialize)]
pub struct ForecastResponse {
    pub list: Vec<Forecast>,
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    #[serde(with = "ts_seconds")]
    pub dt: DateTime<Utc>,
    pub main: Main,
    pub rain: Option<HashMap<String, f32>>,
    pub snow: Option<HashMap<String, f32>>,
}

#[derive(Debug, Deserialize)]
pub struct Main {
    pub feels_like: f32,
    pub humidity: i32,
    pub pressure: i32,
    pub temp: f32,
    pub temp_max: f32,
    pub temp_min: f32,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub description: String,
    pub icon: String,
    pub main: String,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub deg: i32,
    pub gust: Option<f32>,
    pub speed: f32,
}

impl ForecastResponse {
    pub fn dates(&self) -> String {
        let d: BTreeSet<String> = self.list.iter().map(|it| it.dt.format("%d/%m").to_string()).collect();
        d.iter().map(|it| { format!("{}  ", it) }).collect()
    }

    pub fn temps(&self) -> Vec<f32> {
        self.list.iter().map(|f| f.main.temp).collect()
    }

    pub fn prec(&self) -> Vec<f32> {
        self.list.iter().map(|f| f.total_prec()).collect()
    }
}

impl Wind {
    pub fn direction<'a>(&self) -> &'a str {
        let idx = ((self.deg % 360) as f32 / 22.5).round();
        DIRECTIONS[idx as usize]
    }

    pub fn speed_km(&self) -> f32 {
        self.speed * 3.6
    }

    pub fn gust_speed_km(&self) -> f32 {
        match self.gust {
            Some(g) => g * 3.6,
            None => 0f32
        }
    }
}

impl Forecast {
    pub fn total_prec(&self) -> f32 {
        let rain = match &self.rain {
            Some(hm) => hm["3h"],
            None => 0f32
        };
        let snow = match &self.snow {
            Some(hm) => hm["3h"],
            None => 0f32
        };
        rain + snow
    }
}

#[cfg(test)]
mod tests {
    use crate::weather::Wind;

    #[test]
    fn north_0() {
        let w = Wind {
            deg: 0,
            gust: None,
            speed: 0.0,
        };
        assert_eq!("N", w.direction());
    }

    #[test]
    fn north_1() {
        let w = Wind {
            deg: 1,
            gust: None,
            speed: 0.0,
        };
        assert_eq!("N", w.direction());
    }

    #[test]
    fn east_90() {
        let w = Wind {
            deg: 90,
            gust: None,
            speed: 0.0,
        };
        assert_eq!("E", w.direction());
    }

    #[test]
    fn south_180() {
        let w = Wind {
            deg: 180,
            gust: None,
            speed: 0.0,
        };
        assert_eq!("S", w.direction());
    }

    #[test]
    fn south_west_225() {
        let w = Wind {
            deg: 225,
            gust: None,
            speed: 0.0,
        };
        assert_eq!("SW", w.direction());
    }

    #[test]
    fn north_west_north_338() {
        let w = Wind {
            deg: 338,
            gust: None,
            speed: 0.0,
        };
        assert_eq!("NNW", w.direction());
    }

    #[test]
    fn north_360() {
        let w = Wind {
            deg: 360,
            gust: None,
            speed: 0.0,
        };
        assert_eq!("N", w.direction());
    }
}