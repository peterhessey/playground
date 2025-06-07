use crate::app::{App, Coordinate};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::style::Color;

pub fn ui(frame: &mut Frame, app: &App) {
    let text_area = center(
        frame.area(),
        Constraint::Length(app.world_size * 2),
        Constraint::Length(app.world_size),
    );
    let buf = frame.buffer_mut();
    for (idy, y) in (text_area.top()..text_area.bottom()).enumerate() {
        for (idx, x) in (text_area.left()..text_area.right()).enumerate() {
            let mut fg: Color = Color::Yellow;
            let coordinates_to_eval = Coordinate {
                x: idx as u16 / 2,
                y: idy as u16,
            };
            if coordinates_to_eval == app.cube_location {
                fg = Color::Black;
            }
            buf[Position::new(x, y)].set_char('█').set_fg(fg);
        }
    }
}

/// Centers a [`Rect`] within another [`Rect`] using the provided [`Constraint`]s.
fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
