use crate::app::{App, Coordinate};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders};

pub fn ui(frame: &mut Frame, app: &App) {
    let border_area = center(
        frame.area(),
        Constraint::Length(app.world_size * 2 + 2),
        Constraint::Length(app.world_size + 2),
    );
    let game_area = center(
        border_area,
        Constraint::Length(app.world_size * 2),
        Constraint::Length(app.world_size),
    );
    let buf = frame.buffer_mut();

    let border_color = Color::White;
    for y in border_area.top()..border_area.bottom() {
        buf[Position::new(border_area.left(), y)]
            .set_char('█')
            .set_fg(border_color);
        buf[Position::new(border_area.right() - 1, y)]
            .set_char('█')
            .set_fg(border_color);
    }
    for x in border_area.left()..border_area.right() {
        buf[Position::new(x, border_area.top())]
            .set_char('█')
            .set_fg(border_color);
        buf[Position::new(x, border_area.bottom() - 1)]
            .set_char('█')
            .set_fg(border_color);
    }

    let block_color = Color::LightRed;
    for (idy, y) in (game_area.top()..game_area.bottom()).enumerate() {
        for (idx, x) in (game_area.left()..game_area.right()).enumerate() {
            let coordinates_to_eval = Coordinate {
                x: idx as u16 / 2,
                y: idy as u16,
            };
            if coordinates_to_eval == app.cube_location {
                buf[Position::new(x, y)].set_char('█').set_fg(block_color);
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
