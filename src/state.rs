use serde::Deserialize;
use tui_input::Input;

pub enum Focus {
    Input,
    SearchResults,
    History,
    Favorites,
}

pub struct AppState {
    pub city_input: Input,
    pub exit: bool,
    pub focus: Focus,
    pub history: Vec<String>,
    pub history_selected: Option<usize>,
    pub search_results: Vec<CitySearchResult>,
    pub search_selected: Option<usize>,
    pub curren_city: Option<CitySearchResult>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            city_input: Input::new("".to_string()),
            exit: false,
            focus: Focus::Input,
            history: vec![
                "Poltava".to_string(),
                "Wroclaw".to_string(),
                "Kyiv".to_string(),
            ],
            history_selected: Some(0),
            search_results: vec![],
            search_selected: None,
            curren_city: None,
        }
    }

    pub fn app_exit(&mut self) {
        self.exit = true
    }

    pub fn focus_next(&mut self) {
        self.focus = match self.focus {
            Focus::Input => Focus::SearchResults,
            Focus::SearchResults => Focus::History,
            Focus::History => Focus::Favorites,
            Focus::Favorites => Focus::Input,
        }
    }

    pub fn focus_prev(&mut self) {
        self.focus = match self.focus {
            Focus::Input => Focus::Favorites,
            Focus::SearchResults => Focus::Input,
            Focus::History => Focus::SearchResults,
            Focus::Favorites => Focus::History,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GeocodingResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub country: String,
    pub country_code: String,
    pub admin1: Option<String>,
    pub admin2: Option<String>,
    pub timezone: String,
}

#[derive(Deserialize, Debug)]
pub struct GeocodingResponse {
    pub results: Vec<GeocodingResult>,
}

#[derive(Clone)]
pub struct CitySearchResult {
    pub name: String,
    pub country: String,
    pub country_code: String,
    pub admin1: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub is_favorite: bool,
}
