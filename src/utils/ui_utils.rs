use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub accent: Style,
    pub label: Style,
    pub subtle: Style,
    pub error: Style,
    pub default: Style,
    pub border_active: Style,
    pub list_item: Style,
    pub list_highlight: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            accent: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            default: Style::default().fg(Color::White),
            label: Style::default().fg(Color::DarkGray),
            subtle: Style::default().fg(Color::Gray),
            error: Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            border_active: Style::default().fg(Color::Green),
            list_item: Style::default().fg(Color::White),
            list_highlight: Style::default()
                .add_modifier(Modifier::ITALIC)
                .bg(Color::LightYellow),
        }
    }
}

fn glyph_4x5(ch: char) -> [&'static str; 5] {
    match ch {
        '0' => ["███ ", "█ █ ", "█ █ ", "█ █ ", "███ "],
        '1' => [" ██ ", "███ ", " ██ ", " ██ ", "███ "],
        '2' => ["███ ", "  █ ", "███ ", "█   ", "███ "],
        '3' => ["███ ", "  █ ", "███ ", "  █ ", "███ "],
        '4' => ["█ █ ", "█ █ ", "███ ", "  █ ", "  █ "],
        '5' => ["███ ", "█   ", "███ ", "  █ ", "███ "],
        '6' => ["███ ", "█   ", "███ ", "█ █ ", "███ "],
        '7' => ["███ ", "  █ ", "  █ ", "  █ ", "  █ "],
        '8' => ["███ ", "█ █ ", "███ ", "█ █ ", "███ "],
        '9' => ["███ ", "█ █ ", "███ ", "  █ ", "███ "],
        '-' => ["    ", "    ", "███ ", "    ", "    "],
        '.' => ["    ", "    ", "    ", "    ", " ▪  "],
        '°' => [" ▪  ", "▪ ▪ ", " ▪  ", "    ", "    "],
        'C' => ["███ ", "█   ", "█   ", "█   ", "███ "],
        ' ' => ["    ", "    ", "    ", "    ", "    "],
        _ => ["????", "????", "????", "????", "????"],
    }
}

pub fn big_text(s: &str) -> [String; 5] {
    let mut rows = [
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    for ch in s.chars() {
        let g = glyph_4x5(ch);
        for i in 0..5 {
            rows[i].push_str(g[i]);
            rows[i].push(' ');
        }
    }
    rows
}
