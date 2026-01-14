use crate::{
    state::AppState,
    ui::{draw_current_weather, draw_detailed_weather, draw_forecast, history::draw_history},
};

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render(frame: &mut Frame, app: &AppState) {
    let area = frame.area();

    let vertical_center = Layout::vertical([
        Constraint::Min(0),
        Constraint::Length(34),
        Constraint::Min(0),
    ])
    .split(area)[1];

    let centered_area = Layout::horizontal([
        Constraint::Min(0),
        Constraint::Length(100),
        Constraint::Min(0),
    ])
    .split(vertical_center)[1];

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Indexed(244))),
        centered_area,
    );

    let main_layout = Layout::vertical([
        Constraint::Length(12),
        Constraint::Length(8),
        Constraint::Min(0),
        Constraint::Length(2),
    ])
    .margin(2)
    .split(centered_area);

    draw_current_weather(frame, main_layout[0], app);
    draw_detailed_weather(frame, main_layout[1], app);

    let bottom_split = Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(main_layout[2]);

    draw_history(frame, bottom_split[0], app);
    draw_forecast(frame, bottom_split[1], app);

    let border_style = Style::default().fg(Color::Indexed(240));
    frame.render_widget(
        Paragraph::new("[S]earch   [F]avorite   [Q]uit").block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(border_style),
        ),
        main_layout[3],
    );
}
