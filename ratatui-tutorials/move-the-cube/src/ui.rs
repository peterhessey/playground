use crate::app::{App, Coordinate};
use ratatui::Frame;
use ratatui::layout::{Constraint, Flex, Layout, Position, Rect};
use ratatui::prelude::Buffer;
use ratatui::style::Color;

pub fn ui(frame: &mut Frame, app: &App) {
    let border_area = center(
        frame.area(),
        Constraint::Length((app.world_size + 2) * 2),
        Constraint::Length(app.world_size + 2),
    );
    let game_area = center(
        border_area,
        Constraint::Length(app.world_size * 2),
        Constraint::Length(app.world_size),
    );

    let buf = frame.buffer_mut();
    let border_color = Color::White;

    draw_border(buf, border_area, border_color);
    draw_cube(buf, game_area, &app.cube_location);
}

fn draw_border(buf: &mut Buffer, area: Rect, color: Color) {
    // Draw vertical borders
    let vertical_positions = [
        area.left(),
        area.left() + 1,
        area.right() - 2,
        area.right() - 1,
    ];
    for y in area.top()..area.bottom() {
        for &x in &vertical_positions {
            set_cell(buf, x, y, '█', color);
        }
    }

    // Draw horizontal borders
    for x in area.left()..area.right() {
        set_cell(buf, x, area.top(), '█', color);
        set_cell(buf, x, area.bottom() - 1, '█', color);
    }
}

fn draw_cube(buf: &mut Buffer, game_area: Rect, cube_location: &Coordinate) {
    let block_color = Color::LightRed;
    for (idy, y) in (game_area.top()..game_area.bottom()).enumerate() {
        for (idx, x) in (game_area.left()..game_area.right()).enumerate() {
            let coordinates_to_eval = Coordinate {
                x: idx as u16 / 2,
                y: idy as u16,
            };
            if coordinates_to_eval == *cube_location {
                set_cell(buf, x, y, '█', block_color);
            }
        }
    }
}

fn set_cell(buf: &mut Buffer, x: u16, y: u16, ch: char, color: Color) {
    buf[Position::new(x, y)].set_char(ch).set_fg(color);
}

/// Centers a [`Rect`] within another [`Rect`] using the provided [`Constraint`]s.
fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
