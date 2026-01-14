use crate::state::AppState;

use chrono::{NaiveDateTime, Timelike};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    symbols,
    widgets::{Axis, Block, Chart, Dataset, GraphType, Paragraph, Wrap},
};

fn parse_time(s: &str) -> Option<NaiveDateTime> {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M").ok()
}

fn format_hour(s: &str) -> String {
    parse_time(s)
        .map(|dt| dt.format("%H:%M").to_string())
        .unwrap_or_else(|| "??".to_string())
}

fn parse_hour(s: &str) -> Option<f64> {
    parse_time(s).map(|dt| dt.hour() as f64)
}

pub fn draw_detailed_weather(frame: &mut Frame, area: Rect, app: &AppState) {
    let block = Block::default().title("Hourly");

    let Some(weather) = app.weather.as_ref() else {
        let p = Paragraph::new("No hourly data. Select a city and press Enter.")
            .wrap(Wrap { trim: true })
            .block(block);
        frame.render_widget(p, area);
        return;
    };

    let times = &weather.hourly.time;
    let temps = &weather.hourly.temperature;
    let total = times.len().min(temps.len());
    if total == 0 {
        let p = Paragraph::new("Hourly data is empty.")
            .wrap(Wrap { trim: true })
            .block(block);
        frame.render_widget(p, area);
        return;
    }

    let visible = total.min(24);
    let mut points: Vec<(f64, f64)> = Vec::with_capacity(visible);
    for i in 0..visible {
        let x = parse_hour(&times[i]).unwrap_or(i as f64);
        points.push((x, temps[i]));
    }

    let (min_temp, max_temp) = points.iter().fold((temps[0], temps[0]), |acc, p| {
        (acc.0.min(p.1), acc.1.max(p.1))
    });

    let min_x = points.first().map(|p| p.0).unwrap_or(0.0);
    let max_x = points.last().map(|p| p.0).unwrap_or(min_x + 1.0);

    let mut x_labels: Vec<String> = (0..visible)
        .step_by(3)
        .map(|idx| format_hour(&times[idx]))
        .collect();
    if visible > 0 && (visible - 1) % 3 != 0 {
        x_labels.push(format_hour(&times[visible - 1]));
    }

    let chart = Chart::new(vec![
        Dataset::default()
            .marker(symbols::Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(Color::White))
            .data(&points),
    ])
    .block(block)
    .x_axis(
        Axis::default()
            .title("Time")
            .bounds([min_x, max_x])
            .labels(x_labels),
    )
    .y_axis(
        Axis::default()
            .title("Temp")
            .bounds([min_temp.floor() - 1.0, max_temp.ceil() + 1.0])
            .labels(vec![
                format!("{:.0}°", min_temp.floor()),
                format!("{:.0}°", ((min_temp + max_temp) / 2.0).round()),
                format!("{:.0}°", max_temp.ceil()),
            ]),
    );

    frame.render_widget(chart, area);
}
