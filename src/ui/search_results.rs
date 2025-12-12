use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{List, ListDirection};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::state::{AppState, Focus};

pub fn draw_search_results(frame: &mut Frame, area: Rect, app: &AppState) {
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
        .clone()
        .into_iter()
        .map(|city| city.name)
        .collect();

    let list = List::new(cities)
        .block(search_results_block)
        .style(Style::new().fg(Color::White))
        .highlight_style(Style::new().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::BottomToTop);

    frame.render_widget(list, area);
}
