use crate::app::{App, Coordinate};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders};

pub fn ui(frame: &mut Frame, app: &App) {
    let buf = frame.buffer_mut();
    let border_area = center(
        frame.area(),
        Constraint::Length(app.world_size * 2 + 2),
        Constraint::Length(app.world_size + 2),
    );
    for (idy, y) in (border_area.top()..border_area.bottom()).enumerate() {
        for (idx, x) in (border_area.left()..border_area.right()).enumerate() {
            if idx == 0 || idy == y as usize {
                buf[Position::new(x, y)]
                    .set_char('█')
                    .set_fg(Color::LightBlue);
            }
        }
    }

    let game_area = center(
        border_area,
        Constraint::Length(app.world_size * 2),
        Constraint::Length(app.world_size),
    );

    for (idy, y) in (game_area.top()..game_area.bottom()).enumerate() {
        for (idx, x) in (game_area.left()..game_area.right()).enumerate() {
            let coordinates_to_eval = Coordinate {
                x: idx as u16 / 2,
                y: idy as u16,
            };
            if coordinates_to_eval == app.cube_location {
                buf[Position::new(x, y)]
                    .set_char('█')
                    .set_fg(Color::LightBlue);
            }
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
