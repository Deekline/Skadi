use tui_input::Input;

pub enum Focus {
    Input,
    History,
    Favorites,
}

pub struct AppState {
    pub city_input: Input,
    pub exit: bool,
    pub focus: Focus,
    pub history: Vec<String>,
    pub history_selected: Option<usize>,
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
        }
    }

    pub fn app_exit(&mut self) {
        self.exit = true
    }

    pub fn focus_next(&mut self) {
        self.focus = match self.focus {
            Focus::Input => Focus::History,
            Focus::History => Focus::Favorites,
            Focus::Favorites => Focus::Input,
        }
    }

    pub fn focus_prev(&mut self) {
        self.focus = match self.focus {
            Focus::Input => Focus::Favorites,
            Focus::History => Focus::Input,
            Focus::Favorites => Focus::History,
        }
    }
}
