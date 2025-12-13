use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{List, ListDirection, ListState};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::state::{AppState, Focus};

pub fn draw_search_results(frame: &mut Frame, area: Rect, app: &AppState) {
    let mut state = ListState::default();
    state.select(app.search_selected);

    let search_results_block = if matches!(app.focus, Focus::SearchResults) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("Search Results *")
    } else {
        Block::default()
            .borders(Borders::ALL)
            .title("Search Results")
    };

    let cities: Vec<String> = app
        .search_results
        .iter()
        .map(|city| format!("City: {}/ Country: {}", city.name, city.country))
        .collect();

    let list = List::new(cities)
        .block(search_results_block)
        .highlight_style(
            Style::new()
                .add_modifier(Modifier::ITALIC)
                .bg(Color::LightYellow),
        )
        .highlight_symbol(">")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, area, &mut state);
}
