use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::state::AppState;

pub fn draw_forecast(frame: &mut Frame, area: Rect, state: &AppState) {
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Forecast"),
        area,
    );
}
