use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};
use weather::startup::run;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    Ok(result?)
}

struct CityCandidate {
    name: String,
    country: String,
    coordinates: Coordinates,
}

struct FavoriteCity {
    id: String,
    name: String,
    country: String,
    coordinates: Coordinates,
}

struct Coordinates {
    lat: i32,
    lon: i32,
}

struct FavoritesFile {
    version: i8,
    favorites: Vec<FavoriteCity>,
}
