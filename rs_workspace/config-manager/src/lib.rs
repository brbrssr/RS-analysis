use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub symbol: String,
    pub interval: String,
    pub date: String,
    pub min_window: usize,
    pub m_iter: usize,
    pub freq: usize,
    pub alpha: f64,
    pub ub: f64,
    pub hybrid: bool,
    pub max_iters: u64,
    pub herst: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            symbol: "BTCUSD".to_string(),
            interval: "1h".to_string(),
            date: "2025-01-01T00:00:00Z".to_string(),
            min_window: 20,
            m_iter: 100,
            freq: 1,
            alpha: 0.05,
            ub: 1.0,
            hybrid: false,
            max_iters: 1_000_000,
            herst: 0.5,
        }
    }
}

pub struct ConfigManager;

impl ConfigManager {
    const CONFIG_PATH: &'static str = "./config.json";

    pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {

        if let Some(dir) = Path::new(Self::CONFIG_PATH).parent() {
            fs::create_dir_all(dir)?;
        }


        match fs::read_to_string(Self::CONFIG_PATH) {
            Ok(data) if !data.trim().is_empty() => {
                let config: Config = serde_json::from_str(&data)?;
                Ok(config)
            }
            _ => {
                let default = Config::default();
                let serialized = serde_json::to_string_pretty(&default)?;
                let mut file = fs::File::create(Self::CONFIG_PATH)?;
                file.write_all(serialized.as_bytes())?;
                Ok(default)
            }
        }
    }

    pub fn set_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(config)?;
        fs::write(Self::CONFIG_PATH, data)?;
        Ok(())
    }

    pub fn update_symbol(symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Self::get_config()?;
        config.symbol = symbol.to_string();
        Self::set_config(&config)
    }

    pub fn update_interval(interval: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Self::get_config()?;
        config.interval = interval.to_string();
        Self::set_config(&config)
    }

    pub fn update_date(date: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Self::get_config()?;
        config.date = date.to_string();
        Self::set_config(&config)
    }

    pub fn update_herst(h: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Self::get_config()?;
        config.herst = h.parse::<f64>()?;
        Self::set_config(&config)
    }
}
