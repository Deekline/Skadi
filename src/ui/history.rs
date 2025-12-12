use ratatui::style::{Color, Style};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::state::{AppState, Focus};

pub fn draw_history(frame: &mut Frame, area: Rect, app: &AppState) {
    let history_block = if matches!(app.focus, Focus::History) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("History *")
    } else {
        Block::default().borders(Borders::ALL).title("History")
    };

    frame.render_widget(history_block, area);
}
