use chrono::NaiveDate;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::state::AppState;

fn weekday_from_date(date: &str) -> Option<String> {
    let d = NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?;
    Some(d.format("%A").to_string())
}

//TODO move it to utilities module
fn weather_icon_and_label(code: Option<i64>) -> (&'static str, &'static str) {
    match code {
        Some(0) => ("☀️", "Clear"),
        Some(1 | 2) => ("🌤️", "Mostly Clear"),
        Some(3) => ("☁️", "Cloudy"),
        Some(45 | 48) => ("🌫️", "Fog"),
        Some(51 | 53 | 55) => ("🌦️", "Drizzle"),
        Some(61 | 63 | 65) => ("🌧️", "Rain"),
        Some(66 | 67) => ("🌧️❄️", "Freezing Rain"),
        Some(71 | 73 | 75) => ("❄️", "Snow"),
        Some(77) => ("🌨️", "Snow Grains"),
        Some(80 | 81 | 82) => ("🌦️", "Rain Showers"),
        Some(85 | 86) => ("🌨️", "Snow Showers"),
        Some(95) => ("⛈️", "Thunderstorm"),
        Some(96 | 99) => ("⛈️⚡", "Thunderstorm w/ Hail"),
        None => ("?", "Unknown"),
        _ => ("?", "Unknown"),
    }
}

fn push_formated_string(row: &mut String, value: String, width: usize) {
    row.push_str(&format!("{:^width$}", value));
}

pub fn draw_forecast(frame: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default().borders(Borders::ALL).title("Hourly");

    let Some(weather) = state.weather.as_ref() else {
        let p = Paragraph::new("No hourly data. Select a city and press Enter.")
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(block);
        frame.render_widget(p, area);
        return;
    };
    let dates = &weather.daily.date;
    let temp_min = &weather.daily.min_temp;
    let temp_max = &weather.daily.max_temp;
    let weather_code = &weather.daily.weather_code;
    let precipitation_prob = &weather.daily.precipitation_probability_max;
    let precipitation = &weather.daily.precipitation_sum;

    if dates.len() == 0 {
        let p = Paragraph::new("Daily data is empty")
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(block);
        frame.render_widget(p, area);
        return;
    }

    let col_w = 12;
    let max_cols = (area.width / col_w).max(1) as usize;
    let visible = dates.len().min(max_cols);

    let rows: [String; 7] = std::array::from_fn(|_| String::new());
    let [
        mut row_date,
        mut row_temp_min,
        mut row_temp_max,
        mut row_prec_prob,
        mut row_prec,
        mut row_w_icon,
        mut row_desc,
    ] = rows;

    for i in 0..visible {
        let date = weekday_from_date(&dates[i]).unwrap_or("??".to_string());
        push_formated_string(&mut row_date, date, col_w as usize);

        let t_min = temp_min[i].round() as i64;
        let t_min_cell = format!("{t_min}°");
        push_formated_string(&mut row_temp_min, t_min_cell, col_w as usize);

        let t_max = temp_max[i].round() as i64;
        let t_max_cell = format!("{t_max}°");
        push_formated_string(&mut row_temp_max, t_max_cell, col_w as usize);

        let p = precipitation[i];
        let p_cell = format!("{p}%");
        push_formated_string(&mut row_prec, p_cell, col_w as usize);

        let p_prob = precipitation_prob[i];
        let p_prob_cell = format!("{p_prob}%");
        push_formated_string(&mut row_prec_prob, p_prob_cell, col_w as usize);

        let (icon, desc) = weather_icon_and_label(Some(weather_code[i]));

        let w_icon_cell = format!("{icon}");
        push_formated_string(&mut row_w_icon, w_icon_cell, col_w as usize);

        let w_desc_cell = format!("{desc}");
        push_formated_string(&mut row_desc, w_desc_cell, col_w as usize);
    }

    let content = format!(
        "{row_date}\n{row_temp_max}\n{row_w_icon}\n{row_desc}\n{row_temp_min}\n{row_prec_prob}\n{row_prec}"
    );

    let paragraph = Paragraph::new(content)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .block(block);

    frame.render_widget(paragraph, area);
}
