use crate::{
    state::{AppState, weather::CurrentWeather},
    utils::chrono_utils::hhmm,
    utils::ui_utils::{Theme, big_text},
    utils::weather_utils::weather_icon_and_label,
};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

fn build_big_temp_lines(w: &CurrentWeather, theme: &Theme) -> Vec<Line<'static>> {
    let temp_str = format!("{:.1}°C", w.temperature_2m);
    let big = big_text(&temp_str);

    let pad = "  ";
    big.iter()
        .map(|row| Line::from(Span::styled(format!("{pad}{row}"), theme.accent)))
        .collect()
}

fn build_additional_info_lines(w: &CurrentWeather, theme: &Theme) -> Vec<Line<'static>> {
    // Open-Meteo visibility is meters; convert to km (your sample shows 15.7 km)
    let vis_km = w.visibility / 1000.0;

    let wind_dir = w
        .wind_direction_0m
        .map(|d| format!("{d}°"))
        .unwrap_or_else(|| "—".to_string());
    let (icon, desc) = weather_icon_and_label(Some(w.weather_code));

    vec![
        Line::from(vec![
            Span::styled("feels like ", theme.subtle),
            Span::styled(format!("{:.1}°C", w.apparent_temperature), theme.default),
        ]),
        Line::from(Span::styled(format!("{} - {}", icon, desc), theme.subtle)),
        Line::from(""),
        Line::from(vec![
            Span::styled("Wind ", theme.accent),
            Span::styled(format!("{:.1} km/h", w.wind_speed_10m), theme.default),
            Span::styled("  ·  ", theme.label),
            Span::styled("Dir ", theme.label),
            Span::styled(wind_dir, theme.default),
            Span::styled("  ·  ", theme.label),
            Span::styled("Hum ", theme.label),
            Span::styled(format!("{}%", w.relative_humidity_2m), theme.default),
            Span::styled("  ·  ", theme.label),
            Span::styled("Precip ", theme.label),
            Span::styled(format!("{:.1}", w.precipitation), theme.default),
            Span::styled("  ·  ", theme.label),
            Span::styled("Vis ", theme.label),
            Span::styled(format!("{:.1} km", vis_km), theme.default),
        ]),
    ]
}

fn build_details_lines(w: &CurrentWeather, area_width: u16, theme: &Theme) -> Vec<Line<'static>> {
    let vis_km = w.visibility / 1000.0;

    let fill_width = area_width.saturating_sub(2) as usize;
    let mut sep = String::from("- Details ");
    if sep.len() < fill_width {
        sep.push_str(&"─".repeat(fill_width - sep.len()));
    }

    let col2_start = 40usize;

    let left1 = format!("Precip  {:.1}", w.precipitation);
    let left2 = format!("Humidity {}", w.relative_humidity_2m);
    let left3 = format!("Interval {}", w.interval);

    let right1 = format!("Code {}", w.weather_code);
    let right2 = format!("Time {}", hhmm(&w.time));
    let right3 = format!("Vis  {:.1} km", vis_km);

    fn join_two_col(mut left: String, right: String, col2: usize) -> String {
        if left.len() < col2 {
            left.push_str(&" ".repeat(col2 - left.len()));
        } else {
            left.push(' ');
        }
        left.push_str(&right);
        left
    }

    vec![
        Line::from(Span::styled(sep, theme.accent)),
        Line::from(""),
        Line::from(Span::styled(
            join_two_col(left1, right1, col2_start),
            theme.default,
        )),
        Line::from(Span::styled(
            join_two_col(left2, right2, col2_start),
            theme.default,
        )),
        Line::from(Span::styled(
            join_two_col(left3, right3, col2_start),
            theme.default,
        )),
    ]
}

pub fn draw_current_weather(frame: &mut Frame, area: Rect, app: &AppState) {
    let theme = Theme::default();

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
        _ => Line::from("Current Weather"),
    };

    let outer = Block::default().borders(Borders::ALL).title(title);
    frame.render_widget(outer.clone(), area);
    let inner = outer.inner(area);

    if app.current_city.is_none() {
        frame.render_widget(
            Paragraph::new("Select a city and press Enter.").wrap(Wrap { trim: true }),
            inner,
        );
        return;
    }
    if let Some(err) = app.weather_error.as_ref() {
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(Span::styled("Error loading weather:", theme.error)),
                Line::from(err.as_str()),
            ])
            .wrap(Wrap { trim: true }),
            inner,
        );
        return;
    }
    let Some(weather) = app.weather.as_ref() else {
        frame.render_widget(
            Paragraph::new("No weather loaded yet.\nPress Enter to fetch.")
                .wrap(Wrap { trim: true }),
            inner,
        );
        return;
    };

    let w = &weather.current;

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(inner);

    let big_lines = build_big_temp_lines(w, &theme);
    frame.render_widget(Paragraph::new(big_lines), chunks[0]);

    let info_lines = build_additional_info_lines(w, &theme);
    frame.render_widget(
        Paragraph::new(info_lines).wrap(Wrap { trim: true }),
        chunks[1],
    );

    let details_lines = build_details_lines(w, chunks[2].width, &theme);
    frame.render_widget(
        Paragraph::new(details_lines).wrap(Wrap { trim: true }),
        chunks[2],
    );
}
