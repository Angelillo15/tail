use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use log::{error, info};
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub server: Server,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Server {
    pub ip: String,
    pub port: u16,
}

pub fn load(filename: &str) -> Data {
    if !filename.ends_with(".toml") {
        error!("Config file `{}` is not a TOML file", filename);
        exit(1);
    }

    create_if_not_exists(filename);

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            error!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    match toml::from_str(&contents) {
        Ok(d) => {
            info!("Loaded data from `{}`", filename);
            d
        }
        Err(error) => {
            error!("Unable to load data from `{}`", filename);
            error!("Error: {}", error);
            exit(1);
        }
    }
}

fn create_if_not_exists(filename: &str) {
    let file_exists = fs::metadata(filename).is_ok();

    if !file_exists {
        info!("Creating config file `{}`", filename);
        let config = Data {
            server: Server {
                ip: "0.0.0.0".to_string(),
                port: 2222,
            },
        };

        let toml = toml::to_string(&config).unwrap();

        let mut file = File::create(filename).unwrap();
        file.write_all(toml.as_bytes()).unwrap();

        info!("Created config file `{}`", filename);
    }
}