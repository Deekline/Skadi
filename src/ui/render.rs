use crate::{
    state::AppState,
    ui::{
        draw_city_input, favorites::draw_favorites, history::draw_history,
        search_results::draw_search_results,
    },
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders},
};

pub struct LayoutAreas {
    pub bottom_bar_area: Rect,
    pub input_area: Rect,
    pub search_results_area: Rect,
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

    let [
        input_area,
        search_results_area,
        history_area,
        favorites_area,
    ] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(15),
        Constraint::Length(10),
        Constraint::Min(0),
    ])
    .areas(sidebar_area);

    let [current_area, forecast_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(10)]).areas(content_area);

    LayoutAreas {
        bottom_bar_area,
        input_area,
        search_results_area,
        history_area,
        favorites_area,
        current_area,
        forecast_area,
    }
}

pub fn render(frame: &mut Frame, app: &AppState) {
    let areas = compute_layout(frame.area());

    draw_city_input(frame, areas.input_area, app);
    draw_search_results(frame, areas.search_results_area, app);
    draw_history(frame, areas.history_area, app);
    draw_favorites(frame, areas.favorites_area, app);

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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::layout::Rect;

    #[test]
    fn layout_respects_bottom_bar_height_and_sidebar_width() {
        let area = Rect::new(0, 0, 80, 30);

        let layout = compute_layout(area);

        assert_eq!(layout.bottom_bar_area.height, 3);
        assert_eq!(layout.bottom_bar_area.y, 30 - 3);
    }

    #[test]
    fn layout_respects_sidebar_vertical_splits() {
        let area = Rect::new(0, 0, 80, 30);
        let layout = compute_layout(area);

        assert_eq!(layout.input_area.height, 3);
        assert_eq!(layout.history_area.height, 10);
        assert_eq!(
            layout.favorites_area.y,
            layout.history_area.y + layout.history_area.height
        );
    }

    #[test]
    fn layout_respects_content_vertical_splits() {
        let area = Rect::new(0, 0, 80, 30);
        let layout = compute_layout(area);

        assert_eq!(layout.forecast_area.height, 10);
        assert_eq!(
            layout.forecast_area.y,
            layout.current_area.y + layout.current_area.height
        );
    }
}
