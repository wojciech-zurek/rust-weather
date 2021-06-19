use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub api_key: String,
    pub lang: String,
    pub units: String,
    pub latitude: f32,
    pub longitude: f32,
}

const CONFIG_FILE_NAME: &str = "config.json";
const CONFIG_PATH_NAME: &str = "rust-weather";

pub fn config_path() -> PathBuf {
    dirs::config_dir().unwrap().join(CONFIG_PATH_NAME).join(CONFIG_FILE_NAME)
}

fn open_read(path: &PathBuf) -> std::io::Result<File> {
    OpenOptions::new().read(true).open(path)
}

pub fn read() -> Result<Config, Box<dyn std::error::Error>> {
    let path = config_path();
    let file = open_read(&path)?;

    match serde_json::from_reader(file) {
        Ok(config) => Ok(config),
        Err(err) => {
            Err(Box::new(Error::new(ErrorKind::InvalidData, err)))
        }
    }
}



