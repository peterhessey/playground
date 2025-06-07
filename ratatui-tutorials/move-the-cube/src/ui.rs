use crate::app::App;
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::style::Color;
use ratatui::{Frame, text::Text};

pub fn ui(frame: &mut Frame, app: &App) {
    let text_area = center(
        frame.area(),
        Constraint::Length(app.world_size * 2),
        Constraint::Length(1),
    );

    let buf = frame.buffer_mut();
    for (idx, x) in (text_area.left()..text_area.right()).enumerate() {
        let mut fg: Color = Color::Yellow;
        if idx as u16 / 2 == app.cube_location {
            fg = Color::Black;
        }
        buf[Position::new(x, text_area.y)].set_char('█').set_fg(fg);
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
