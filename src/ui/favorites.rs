use ratatui::widgets::{List, ListDirection, ListState, Paragraph, Wrap};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::state::{AppState, Focus};
use crate::utils::ui_utils::Theme;

pub fn draw_favorites(frame: &mut Frame, area: Rect, app: &AppState) {
    let theme = Theme::default();

    let favorites_block = if matches!(app.focus, Focus::Favorites) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme.border_active)
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
            .style(theme.list_item)
            .highlight_style(theme.list_highlight)
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
