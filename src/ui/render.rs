use crate::{
    state::AppState,
    ui::{draw_current_weather, draw_detailed_weather, draw_forecast, history::draw_history},
    utils::{chrono_utils::hhmm, ui_utils::Theme},
};

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

pub fn render(frame: &mut Frame, app: &AppState) {
    let area = frame.area();
    let theme = Theme::default();

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

    let title = match (app.current_city.as_ref(), app.weather.as_ref()) {
        (Some(city), Some(w)) => Line::from(vec![
            Span::styled(format!("{}, {}", city.name, city.country), theme.accent),
            Span::raw("  "),
            Span::styled(format!("Updated {}", hhmm(&w.current.time)), theme.default),
        ]),
        (Some(city), None) => Line::from(Span::styled(
            format!("{}, {}", city.name, city.country),
            theme.accent,
        )),
        _ => Line::from("Weather"),
    };

    let outer = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Indexed(244)))
        .title(title);

    frame.render_widget(outer, centered_area);

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
    draw_forecast(frame, main_layout[2], app);
    let border_style = Style::default().fg(Color::Indexed(240));
    frame.render_widget(
        Paragraph::new("[S]earch   [H]istory   [F]avorite   [Q]uit").block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(border_style),
        ),
        main_layout[3],
    );

    if app.history_popup {
        let popup_area = centered_rect(area, 20, 20); // 60% width, 50% height

        frame.render_widget(Clear, popup_area);

        draw_history(frame, popup_area, app);
    }
}

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
