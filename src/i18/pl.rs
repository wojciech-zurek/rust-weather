use std::fmt::{Write};
use crate::weather::{WeatherResponse, ForecastResponse};
use std::fmt;
use crate::spark::graph;

pub trait PL {
    fn translate(&self) -> Result<String, fmt::Error>;
}

impl PL for WeatherResponse {
    fn translate(&self) -> Result<String, fmt::Error> {
        let mut o = String::new();
        writeln!(o, "Pogoda dla {}, {}", self.name, self.sys.country)?;
        writeln!(o)?;
        writeln!(o, " Temperatura: {}°C ({}°C)", self.main.temp, self.main.feels_like)?;
        writeln!(o, "  Wilgotność: {}%", self.main.humidity)?;
        writeln!(o, "Zachmurzenie: {}", self.weather[0].description)?;
        writeln!(o, "   Ciśnienie: {} hPa", self.main.pressure)?;
        write!(o, "       Wiatr: z {} ({}°) {:.2} km/h",
               self.wind.direction(), self.wind.deg, self.wind.speed_km())?;

        match self.wind.gust {
            Some(_) => writeln!(o, " w porywach do {:.2} km/h", self.wind.gust_speed_km())?,
            None => writeln!(o)?
        };

        Ok(o)
    }
}

impl PL for ForecastResponse {
    fn translate(&self) -> Result<String, fmt::Error> {
        let dates = self.dates();

        let mut o = String::new();

        writeln!(o, "Prognoza na 5 dni")?;
        write!(o, " Temperatura: ")?;

        writeln!(o, "{}", graph(&self.temps()))?;
        writeln!(o, "              {}", &dates)?;

        writeln!(o)?;

        write!(o, "       Opady: ")?;
        writeln!(o, "{}", graph(&self.prec()))?;
        writeln!(o, "              {}", &dates)?;
        Ok(o)
    }
}
