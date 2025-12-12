use crate::state::AppState;
use ratatui::{
    Frame,
    layout::Alignment,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw_city_input(frame: &mut Frame, area: Rect, app: &AppState) {
    let paragraph = Paragraph::new(format!("City: {}", app.city_input.value()))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::all())
                .title("Search city"),
        );
    frame.render_widget(paragraph, area);
}
