use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{List, ListDirection};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

use crate::state::{AppState, Focus};

pub fn draw_history(frame: &mut Frame, area: Rect, app: &AppState) {
    //let history = app.history.iter().enumerate();

    let history_block = if matches!(app.focus, Focus::History) {
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title("History *")
    } else {
        Block::default().borders(Borders::ALL).title("History")
    };
    let list = List::new(app.history.clone())
        .block(history_block)
        .style(Style::new().fg(Color::White))
        .highlight_style(Style::new().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::BottomToTop);

    frame.render_widget(list, area);
}
