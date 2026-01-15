use tui_input::Input;

use crate::{
    configuration::Config,
    state::{CitySearchResult, Weather},
};

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
    pub history: Vec<CitySearchResult>,
    pub history_selected: Option<usize>,
    pub search_results: Vec<CitySearchResult>,
    pub search_selected: Option<usize>,
    pub current_city: Option<CitySearchResult>,
    pub weather: Option<Weather>,
    pub weather_error: Option<String>,
    pub favorite: Option<CitySearchResult>,
    pub history_popup: bool,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            city_input: Input::new("".to_string()),
            exit: false,
            focus: Focus::Input,
            history: config.history,
            history_selected: Some(0),
            search_results: vec![],
            search_selected: None,
            current_city: None,
            weather: None,
            weather_error: None,
            favorite: config.favorite,
            history_popup: false,
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
