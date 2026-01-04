use crate::state::CitySearchResult;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub favorite: Option<CitySearchResult>,
    pub history: Vec<CitySearchResult>,
}

impl Config {
    fn config_path() -> PathBuf {
        dirs::home_dir()
            .expect("No home dir")
            .join(".config/skadi/skadi.ron")
    }

    pub fn load_config() -> Self {
        let path = Self::config_path();
        match fs::read_to_string(&path) {
            Ok(data) => ron::from_str(&data).unwrap_or_default(),
            Err(_) => Config::default(),
        }
    }

    pub fn save_config(&self) -> std::io::Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data = ron::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        fs::write(path, data)?;
        Ok(())
    }

    pub fn save_history(city: CitySearchResult) -> std::io::Result<()> {
        let mut cfg = Self::load_config();

        cfg.history.retain(|c| c != &city);
        cfg.history.insert(0, city);

        cfg.history.truncate(20);

        cfg.save_config()
    }
}
