use tui_input::Input;

enum Focus {
    Input,
    History,
    Favorites,
    Forecast,
    None,
}

pub struct AppState {
    pub city_input: Input,
    pub exit: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            city_input: Input::new("".to_string()),
            exit: false,
        }
    }

    pub fn app_exit(&mut self) {
        self.exit = true
    }
}
