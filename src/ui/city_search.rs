use crate::state::{AppState, Focus};
use crate::utils::ui_utils::Theme;
use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::BorderType;
use ratatui::{
    Frame,
    layout::{Alignment, Position, Rect},
    widgets::{Block, Borders, List, ListDirection, ListState, Paragraph, Wrap},
};

pub fn draw_city_search(frame: &mut Frame, area: Rect, app: &AppState) {
    let theme = Theme::default();

    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border_default);

    frame.render_widget(outer, area);

    let layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)])
        .margin(2)
        .split(area);

    // SearchInput
    let paragraph = Paragraph::new(format!("City: {}", app.city_input.value()))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .block(Block::default());

    frame.render_widget(paragraph, layout[0]);

    if matches!(app.focus, Focus::Input) {
        let label = "City: ";
        let label_len = label.chars().count() as u16;
        let value_len = app.city_input.cursor() as u16;

        let x = area.x + 2 + label_len + value_len;
        let y = area.y + 2;
        let position = Position { x, y };

        frame.set_cursor_position(position);
    }

    // SearchResults

    let mut state = ListState::default();
    state.select(app.search_selected);

    let search_results_block = if matches!(app.focus, Focus::SearchResults) {
        Block::default()
            .borders(Borders::TOP)
            .title("Search Results * ")
    } else {
        Block::default()
            .borders(Borders::TOP)
            .title("Search Results ")
    };

    let cities: Vec<String> = app
        .search_results
        .iter()
        .map(|city| format!("City: {}/ Country: {}", city.name, city.country))
        .collect();

    let list = List::new(cities)
        .block(search_results_block)
        .style(theme.list_item)
        .highlight_style(theme.list_highlight)
        .highlight_symbol(">")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    frame.render_stateful_widget(list, layout[1], &mut state);
}
