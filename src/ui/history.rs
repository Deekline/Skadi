use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{List, ListDirection, ListState};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::state::{AppState, Focus};

pub fn draw_history(frame: &mut Frame, area: Rect, app: &AppState) {
    let mut state = ListState::default();
    let selected = app.history_selected.filter(|&idx| idx < app.history.len());
    state.select(selected);

    let history_block = if matches!(app.focus, Focus::History) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("History *")
    } else {
        Block::default().borders(Borders::ALL).title("History")
    };

    let cities: Vec<String> = app
        .history
        .iter()
        .map(|city| format!("City: {}/ Country: {}", city.name, city.country))
        .collect();

    let list = List::new(cities)
        .block(history_block)
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
}
