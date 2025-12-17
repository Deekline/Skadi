use crate::state::AppState;
use ratatui::layout::Alignment;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

fn extract_hour(s: &str) -> Option<&str> {
    let t_pos = s.rfind('T')?;
    let rest = &s[t_pos + 1..];
    if rest.len() < 2 {
        return None;
    }
    Some(rest)
}

pub fn draw_detailed_weather(frame: &mut Frame, area: Rect, app: &AppState) {
    let block = Block::default().borders(Borders::ALL).title("Hourly");

    let Some(weather) = app.weather.as_ref() else {
        let p = Paragraph::new("No hourly data. Select a city and press Enter.")
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(block);
        frame.render_widget(p, area);
        return;
    };
    let times = &weather.hourly.time;
    let temps = &weather.hourly.temperature;
    let humidity = &weather.hourly.humidity;

    let total = times.len().min(temps.len());
    if total == 0 {
        let p = Paragraph::new("Hourly data is empty.")
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(block);
        frame.render_widget(p, area);
        return;
    }

    let col_w: u16 = 6;
    let max_cols = (area.width / col_w).max(1) as usize;
    let visible = total.min(max_cols);

    let mut row_time = String::new();
    let mut row_mid = String::new();
    let mut row_temp = String::new();
    let mut row_hum = String::new();

    for i in 0..visible {
        let hour = extract_hour(&times[i]).unwrap_or("??");

        row_time.push_str(&format!("{:^width$}", hour, width = col_w as usize));
        row_mid.push_str(&" ".repeat(col_w as usize));

        let t = temps[i].round() as i64;
        let t_cell = format!("{t}°");
        row_temp.push_str(&format!("{:^width$}", t_cell, width = col_w as usize));

        let h = humidity[i];
        let h_cell = format!("{h}%");
        row_hum.push_str(&format!("{:^width$}", h_cell, width = col_w as usize));
    }

    let content = format!("{row_time}\n{row_mid}\n{row_temp}\n{row_hum}");

    let paragraph = Paragraph::new(content)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false })
        .block(block);

    frame.render_widget(paragraph, area);
}
