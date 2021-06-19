use std::fmt::{Write};
use crate::weather::{WeatherResponse, ForecastResponse};
use std::fmt;
use crate::spark::graph;

pub trait EN {
    fn translate(&self) -> Result<String, fmt::Error>;
}

impl EN for WeatherResponse {
    fn translate(&self) -> Result<String, fmt::Error> {
        let mut o = String::new();
        writeln!(o, "Weather for {}, {}", self.name, self.sys.country)?;
        writeln!(o)?;
        writeln!(o, "  Temperature: {}°C ({}°C)", self.main.temp, self.main.feels_like)?;
        writeln!(o, "     Humidity: {}%", self.main.humidity)?;
        writeln!(o, "   Cloudiness: {}", self.weather[0].description)?;
        writeln!(o, "     Pressure: {} hPa", self.main.pressure)?;
        write!(o, "         Wind: from {} ({}°) at {:.2} km/h",
               self.wind.direction(), self.wind.deg, self.wind.speed_km())?;

        match self.wind.gust {
            Some(_) => writeln!(o, " gusting to {:.2} km/h", self.wind.gust_speed_km())?,
            None => writeln!(o)?
        };

        Ok(o)
    }
}

impl EN for ForecastResponse {
    fn translate(&self) -> Result<String, fmt::Error> {
        let dates = self.dates();

        let mut o = String::new();

        writeln!(o, "5-day forecast")?;
        write!(o, "  Temperature: ")?;

        writeln!(o, "{}", graph(&self.temps()))?;
        writeln!(o, "               {}", &dates)?;

        writeln!(o)?;

        write!(o, "Precipitation: ")?;
        writeln!(o, "{}", graph(&self.prec()))?;
        writeln!(o, "               {}", &dates)?;
        Ok(o)
    }
}