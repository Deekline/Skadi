use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{List, ListDirection, ListState, Paragraph, Wrap};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::state::{AppState, Focus};

pub fn draw_favorites(frame: &mut Frame, area: Rect, app: &AppState) {
    let favorites_block = if matches!(app.focus, Focus::Favorites) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("Favorites *")
    } else {
        Block::default().borders(Borders::ALL).title("Favorites")
    };

    if let Some(city) = app.favorite.as_ref() {
        let mut state = ListState::default();
        if matches!(app.focus, Focus::Favorites) {
            state.select(Some(0));
        }

        let cities = vec![format!("City: {}/ Country: {}", city.name, city.country)];

        let list = List::new(cities)
            .block(favorites_block)
            .style(Style::new().fg(Color::White))
            .highlight_style(
                Style::new()
                    .add_modifier(Modifier::ITALIC)
                    .bg(Color::LightYellow),
            )
            .highlight_symbol(">")
            .repeat_highlight_symbol(true)
            .direction(ListDirection::TopToBottom);

        frame.render_stateful_widget(list, area, &mut state);
    } else {
        let paragraph = Paragraph::new("No favorite city set.")
            .wrap(Wrap { trim: true })
            .block(favorites_block);
        frame.render_widget(paragraph, area);
    }
}
