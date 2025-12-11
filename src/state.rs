use tui_input::Input;

pub struct CityInputState {
    pub input: Input,
    pub focused: bool,
}

impl CityInputState {
    pub fn new() -> Self {
        Self {
            input: Input::new("Poltava".to_string()),
            focused: true,
        }
    }
}

pub struct AppState {
    pub city_input: CityInputState,
    pub exit: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            city_input: CityInputState::new(),
            exit: false,
        }
    }

    pub fn focus_input(&mut self) {
        self.city_input.focused = true;
    }

    pub fn blur_input(&mut self) {
        self.city_input.focused = false;
    }

    pub fn app_exit(&mut self) {
        self.exit = true
    }
}
