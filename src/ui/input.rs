use crate::state::AppState;
use ratatui::{
    DefaultTerminal, Frame,
    layout::Alignment,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn draw_city_input(frame: &mut Frame, app: &AppState) {
    let vertical = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let [input_area, forecast_area] = vertical.areas(frame.area());
    frame.render_widget(
        Paragraph::new(format!("City: {}", app.city_input.input.value()))
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::all())
                    .title("Search city"),
            ),
        input_area,
    );
}
