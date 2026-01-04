use crate::state::CitySearchResult;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub favorite: Option<CitySearchResult>,
    pub history: Vec<CitySearchResult>,
}

pub fn load_config() -> Config {
    let mut candidates = Vec::new();
    if let Some(config_dir) = dirs::config_dir() {
        candidates.push(config_dir.join("skadi/skadi.ron"));
    }
    if let Some(home_dir) = dirs::home_dir() {
        candidates.push(home_dir.join(".config/skadi/skadi.ron"));
    }

    for path in candidates {
        if let Ok(data) = std::fs::read_to_string(&path) {
            return ron::from_str(&data).unwrap_or_default();
        }
    }

    Config {
        favorite: None,
        history: vec![],
    }
}
