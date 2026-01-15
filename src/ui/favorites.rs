use ratatui::widgets::{Paragraph, Wrap};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

pub fn draw_favorites(frame: &mut Frame, area: Rect) {
    let favorites_block = Block::default().borders(Borders::ALL).title("Favorites");

    let paragraph = Paragraph::new("No favorite city set.")
        .wrap(Wrap { trim: true })
        .block(favorites_block);
    frame.render_widget(paragraph, area);
}
