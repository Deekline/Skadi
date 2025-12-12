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
    pub selected: Option<usize>,
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
            selected: None,
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

pub struct GeocodingResult {
    id: u32,
    name: String,
    latitude: f32,
    longitude: f32,
    elevation: Option<f64>,
    feature_code: String,
    country_code: String,
    admin1_id: u32,
    admin2_id: u32,
    timezone: String,
    population: u32,
    country_id: u32,
    country: String,
    admin1: Option<String>,
    admin2: Option<String>,
}

pub struct GeocodingResponse {
    result: Vec<GeocodingResult>,
}

#[derive(Clone)]
pub struct CitySearchResult {
    name: String,
    country: String,
    country_code: String,
    admin1: Option<String>,
    lat: f64,
    lon: f64,
    elevation: Option<f64>,
    timezone: String,
    is_favorite: bool,
}
