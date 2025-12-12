use crate::{state::AppState, ui::draw_city_input};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

pub fn render(frame: &mut Frame, app: &AppState) {
    let [main_area, _bottom_bar_area] =
        Layout::vertical([Constraint::Min(0), Constraint::Length(3)]).areas(frame.area());

    let [sidebar_area, _content_area] =
        Layout::horizontal([Constraint::Length(50), Constraint::Min(0)]).areas(main_area);

    let [input_area, history_area, favorites_area] = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(5),
        Constraint::Min(0),
    ])
    .areas(sidebar_area);

    draw_city_input(frame, input_area, app);
}
