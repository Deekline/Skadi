use crate::state::AppState;
use ratatui::layout::Alignment;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

fn weather_code_to_text() {
    todo!();
}

fn deg_to_compas() {
    todo!();
}

pub fn draw_current_weather(frame: &mut Frame, area: Rect, app: &AppState) {
    let block_title = match app.current_city.as_ref() {
        Some(city) => format!("Current Weather — {}", city.name),
        None => "Current Weather".to_string(),
    };

    let weather_block = Block::default().borders(Borders::ALL).title(block_title);

    let content = if app.current_city.is_none() {
        "Select a city and press Enter.".to_string()
    } else if let Some(err) = app.weather_error.as_ref() {
        format!("Error loading weather:\n{err}")
    } else if app.weather.is_none() {
        "No weather loaded yet.\nPress Enter to fetch.".to_string()
    } else {
        let w = app.weather.as_ref().unwrap();
        format!(
            "Temperature: {}°C\nFeels like: {}°C\nHumidity: {}%\nWind: {} km/h\n",
            w.current.temperature_2m,
            w.current.apparent_temperature,
            w.current.relative_humidity_2m,
            w.current.wind_speed_10m,
        )
    };

    let temp_paragraph = Paragraph::new(content)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .block(weather_block);

    frame.render_widget(temp_paragraph, area);
}
