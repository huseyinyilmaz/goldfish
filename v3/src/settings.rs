use config::{Config, ConfigError, Environment, File};
use log::debug;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Deserialize, Serialize)]
pub struct SettingsOptions {
    pub ip_address: Option<IpAddr>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub ip_address: IpAddr,
    pub port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            ip_address: "0.0.0.0".parse().unwrap(),
            port: 11211,
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        debug!("Reading settings.");
        let mut config_builder = Config::builder();
        // Start off by merging in the "default" configuration file
        config_builder = config_builder.add_source(File::with_name("goldfish").required(false));
        config_builder = config_builder.add_source(Environment::with_prefix("goldfish"));
        let default: Settings = Settings::default();
        let parsed_config = config_builder.build()?;
        let parsed_settings: SettingsOptions = parsed_config.try_deserialize()?;
        let settings = Settings {
            ip_address: parsed_settings.ip_address.unwrap_or(default.ip_address),
            port: parsed_settings.port.unwrap_or(default.port),
        };
        debug!("Settings = {:?}", &settings);
        Ok(settings)
    }
}
