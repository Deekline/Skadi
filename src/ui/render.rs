use crate::{state::AppState, ui::draw_city_input};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
};

pub struct LayoutAreas {
    pub bottom_bar_area: Rect,
    pub input_area: Rect,
    pub history_area: Rect,
    pub favorites_area: Rect,
    pub current_area: Rect,
    pub forecast_area: Rect,
}

fn compute_layout(area: Rect) -> LayoutAreas {
    let [main_area, bottom_bar_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(area);

    let [sidebar_area, content_area] =
        Layout::horizontal([Constraint::Length(50), Constraint::Min(0)]).areas(main_area);

    let [input_area, history_area, favorites_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(10),
        Constraint::Min(0),
    ])
    .areas(sidebar_area);

    let [current_area, forecast_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(10)]).areas(content_area);

    LayoutAreas {
        bottom_bar_area,
        input_area,
        history_area,
        favorites_area,
        current_area,
        forecast_area,
    }
}

pub fn render(frame: &mut Frame, app: &AppState) {
    let areas = compute_layout(frame.area());

    draw_city_input(frame, areas.input_area, app);

    // Placeholder
    frame.render_widget(
        Block::default().borders(Borders::ALL).title("History"),
        areas.history_area,
    );

    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Favorites"),
        areas.favorites_area,
    );

    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Current"),
        areas.current_area,
    );

    frame.render_widget(
        Block::default().borders(Borders::ALL).title("Forecast"),
        areas.forecast_area,
    );

    frame.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title("Help / Navigation"),
        areas.bottom_bar_area,
    );
}
