use crate::state::{GeoCoordinates, History};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Favorite {
    name: String,
    country: String,
    coordinates: GeoCoordinates,
}

#[derive(Deserialize, Serialize, Default)]
struct Config {
    favorite: Option<Favorite>,
    history: Vec<History>,
}

fn load_config() -> Config {
    let path = dirs::config_dir().unwrap().join("skadi/skadi.ron");
    if let Ok(data) = std::fs::read_to_string(&path) {
        ron::from_str(&data).unwrap_or_default()
    } else {
        Config {
            favorite: None,
            history: vec![],
        }
    }
}

use std::collections::{HashMap, HashSet};
struct Solution {}

impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut trans: HashMap<(u8, u8), Vec<u8>> = HashMap::new();
        println!("{}", format!("{:?}", allowed));
        for s in allowed {
            let b = s.as_bytes();
            let key = (b[0], b[1]);
            trans.entry(key).or_default().push(b[2]);
        }
        let row = bottom.into_bytes();
        Self::dfs(&row, &trans)
    }

    pub fn dfs(row: &[u8], trans: &HashMap<(u8, u8), Vec<u8>>) -> bool {
        if row.len() == 1 {
            return true;
        }

        let mut next: Vec<u8> = Vec::with_capacity(row.len() - 1);

        Self::build(0, row, &mut next, trans)
    }

    pub fn build(
        i: usize,
        row: &[u8],
        next: &mut Vec<u8>,
        trans: &HashMap<(u8, u8), Vec<u8>>,
    ) -> bool {
        if i == row.len() - 1 {
            return Self::dfs(next, trans);
        }

        let key = (row[i], row[i + 1]);
        let Some(options) = trans.get(&key) else {
            return false;
        };

        for &ch in options {
            next.push(ch);

            if Self::build(i + 1, row, next, trans) {
                return true;
            }
            next.pop();
        }
        return false;
    }
}
