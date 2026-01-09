use crate::{
    state::{AppState, weather::CurrentWeather},
    utils::ui_utils::big_text,
};

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

//TODO  combine this with existing one and move to utils
fn weather_code_label(code: i64) -> &'static str {
    match code {
        0 => "Clear",
        1 | 2 => "Mainly clear",
        3 => "Overcast",
        45 | 48 => "Fog",
        51 | 53 | 55 => "Drizzle",
        61 | 63 | 65 => "Rain",
        71 | 73 | 75 => "Snow",
        80 | 81 | 82 => "Showers",
        95 => "Thunderstorm",
        96 | 99 => "Thunderstorm (hail)",
        _ => "Unknown",
    }
}

fn hhmm(s: &str) -> String {
    if s.len() >= 16 {
        let t = &s[11..16];
        if t.chars().nth(2) == Some(':') {
            return t.to_string();
        }
    }
    s.to_string()
}

fn build_big_temp_lines(w: &CurrentWeather) -> Vec<Line<'static>> {
    let accent = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);

    let temp_str = format!("{:.1}°C", w.temperature_2m);
    let big = big_text(&temp_str);

    let pad = "  ";
    big.iter()
        .map(|row| Line::from(Span::styled(format!("{pad}{row}"), accent)))
        .collect()
}

fn build_additional_info_lines(w: &CurrentWeather) -> Vec<Line<'static>> {
    let accent = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let label = Style::default().fg(Color::DarkGray);
    let value = Style::default().fg(Color::White);
    let subtle = Style::default().fg(Color::Gray);

    // Open-Meteo visibility is meters; convert to km (your sample shows 15.7 km)
    let vis_km = w.visibility / 1000.0;

    let wind_dir = w
        .wind_direction_0m
        .map(|d| format!("{d}°"))
        .unwrap_or_else(|| "—".to_string());

    vec![
        Line::from(vec![
            Span::styled("feels like ", subtle),
            Span::styled(format!("{:.1}°C", w.apparent_temperature), value),
        ]),
        Line::from(Span::styled(weather_code_label(w.weather_code), subtle)),
        Line::from(""),
        Line::from(vec![
            Span::styled("Wind ", accent),
            Span::styled(format!("{:.1} km/h", w.wind_speed_10m), value),
            Span::styled("  ·  ", label),
            Span::styled("Dir ", label),
            Span::styled(wind_dir, value),
            Span::styled("  ·  ", label),
            Span::styled("Hum ", label),
            Span::styled(format!("{}%", w.relative_humidity_2m), value),
            Span::styled("  ·  ", label),
            Span::styled("Precip ", label),
            Span::styled(format!("{:.1}", w.precipitation), value),
            Span::styled("  ·  ", label),
            Span::styled("Vis ", label),
            Span::styled(format!("{:.1} km", vis_km), value),
        ]),
    ]
}

fn build_details_lines(w: &CurrentWeather, area_width: u16) -> Vec<Line<'static>> {
    let accent = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let value = Style::default().fg(Color::White);

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
        Line::from(Span::styled(sep, accent)),
        Line::from(""),
        Line::from(Span::styled(join_two_col(left1, right1, col2_start), value)),
        Line::from(Span::styled(join_two_col(left2, right2, col2_start), value)),
        Line::from(Span::styled(join_two_col(left3, right3, col2_start), value)),
    ]
}

pub fn draw_current_weather(frame: &mut Frame, area: Rect, app: &AppState) {
    let title = match (app.current_city.as_ref(), app.weather.as_ref()) {
        (Some(city), Some(w)) => Line::from(vec![
            Span::styled(
                format!("{}, {}", city.name, city.country),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                format!("Updated {}", hhmm(&w.current.time)),
                Style::default().fg(Color::White),
            ),
        ]),
        (Some(city), None) => Line::from(Span::styled(
            format!("{}, {}", city.name, city.country),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
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
                Line::from(Span::styled(
                    "Error loading weather:",
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                )),
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
            Constraint::Length(5), // big font height (adjust if your big_text is 3/5 lines)
            Constraint::Length(5), // additional info area
            Constraint::Min(0),    // details
        ])
        .split(inner);

    let big_lines = build_big_temp_lines(w);
    frame.render_widget(Paragraph::new(big_lines), chunks[0]);

    let info_lines = build_additional_info_lines(w);
    frame.render_widget(
        Paragraph::new(info_lines).wrap(Wrap { trim: true }),
        chunks[1],
    );

    let details_lines = build_details_lines(w, chunks[2].width);
    frame.render_widget(
        Paragraph::new(details_lines).wrap(Wrap { trim: true }),
        chunks[2],
    );
}
