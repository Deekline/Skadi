use crate::state::{AppState, Focus};
use ratatui::{
    Frame,
    layout::{Alignment, Position, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw_city_input(frame: &mut Frame, area: Rect, app: &AppState) {
    let paragraph = Paragraph::new(format!("City: {}", app.city_input.value()))
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::all())
                .title("Search city"),
        );

    frame.render_widget(paragraph, area);

    if matches!(app.focus, Focus::Input) {
        let label = "City: ";
        let label_len = label.chars().count() as u16;
        let value_len = app.city_input.cursor() as u16;

        let x = area.x + 1 + label_len + value_len;
        let y = area.y + 1;
        let position = Position { x, y };

        frame.set_cursor_position(position);
    }
}
