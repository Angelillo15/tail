use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process::exit;
use log::{debug, error, info};
use serde_toml_merge::{merge};
use toml;
use toml::Value;

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub server: Server,
    pub database_url: String,
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

    update_config(&contents, filename);

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

fn update_config(content: &str, filename: &str) {
    let content = content.parse::<Value>().unwrap();
    let config = toml::to_string(&get_default_config()).unwrap().parse::<Value>().unwrap();

    let result = match merge(content, config) {
        Ok(result) => {
            debug!("Config updated successfully");
            result
        }
        Err(error) => {
            error!("Unable to update config");
            drop(error);
            exit(1);
        }
    };

    let final_config = match toml::to_string(&result) {
        Ok(c) => c,
        Err(_) => {
            error!("Unable to convert config to string");
            exit(1);
        }
    };

    let path = fs::canonicalize(filename).unwrap();

    match OpenOptions::new().write(true).open(&path) {
        Ok(mut file) => {
            match file.write_all(final_config.as_bytes()) {
                Ok(_) => {}
                Err(error) => {
                    error!("Unable to write to file `{}`", &path.as_path().to_str().unwrap());
                    error!("{}", error);
                    exit(1);
                }
            };
            debug!("Config update written to file");
        }
        Err(_) => {
            error!("Unable to open file `{}`", filename);
            exit(1);
        }
    };
}

fn create_if_not_exists(filename: &str) {
    let file_exists = fs::metadata(filename).is_ok();

    if !file_exists {
        info!("Creating CONFIG file `{}`", filename);

        let toml = toml::to_string(&get_default_config()).unwrap();

        let mut file = File::create(filename).unwrap();
        file.write_all(toml.as_bytes()).unwrap();

        info!("Created CONFIG file `{}`", filename);
    }
}

fn get_default_config() -> Data {
    Data {
        database_url: "mysql://root:password@localhost:3306/tail".to_string(),
        server: Server {
            ip: "0.0.0.0".to_string(),
            port: 2222,
        },
    }
}