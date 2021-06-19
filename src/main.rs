mod weather;
mod spark;
mod config;
mod i18;

use crate::weather::{WeatherResponse, ForecastResponse};
use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::config::Config;
use crate::i18::en::EN;
use crate::i18::pl::PL;

// Weather for Rzeszów, PL
//
// Temperature: 28.72°C (28.25°C)
// Humidity: 39%
// Cloudiness: clear sky
// Pressure: 1006 hPa
// Wind: from SE (99°) at 9.65 km/h gusting to 17.71 km/h
//
// 5-day forecast
// Temperature: ▆▃▁▁▃▅▆▅▃▂▁▁▃▅▅▅▄▂▁▁▃▅▆▆▄▂▂▂▄▆▇▇▅▃▃▂▅▆█▇
// 18/06  19/06  20/06  21/06  22/06  23/06
//
// Precipitation: ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▃▅▁▂▂▂▄▁▁█▁▁▁▁▁▁
// 18/06  19/06  20/06  21/06  22/06  23/06


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::read()?;

    let client = reqwest::Client::new();
    let req_w = Request::new(client.clone(), "weather", &config);
    let req_f = Request::new(client, "forecast", &config);

    let weather = tokio::spawn(async move {
        match call::<WeatherResponse>(&req_w).await {
            Ok(r) => match req_w.lang.as_str() {
                "pl" => PL::translate(&r).unwrap(),
                _ => EN::translate(&r).unwrap()
            }
            Err(e) => format!("{:?}", e)
        }
    });
    let forecast = tokio::spawn(async move {
        match call::<ForecastResponse>(&req_f).await {
            Ok(r) => match req_f.lang.as_str() {
                "pl" => PL::translate(&r).unwrap(),
                _ => EN::translate(&r).unwrap()
            }
            Err(e) => format!("{:?}", e)
        }
    });

    println!("{}", weather.await?);
    println!("{}", forecast.await?);

    Ok(())
}

async fn call<T: DeserializeOwned>(request: &Request) -> Result<T, Box<dyn std::error::Error>> {
    let client = &request.client;
    let resp: T = client
        .get(&request.url)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}

struct Request {
    url: String,
    client: Client,
    lang: String,
    units: String,
}

impl Request {
    fn new(client: Client, api_type: &str, config: &Config) -> Self {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/{}?lat={}&lon={}&units={}&lang={}&appid={}",
            api_type, config.latitude, config.longitude, config.units, config.lang, config.api_key);
        Request {
            client,
            url,
            lang: config.lang.clone(),
            units: config.units.clone(),
        }
    }
}



