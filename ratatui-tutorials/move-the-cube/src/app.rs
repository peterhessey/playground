#[derive(Eq, PartialEq)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}

pub struct App {
    pub world_size: u16,
    pub cube_location: Coordinate,
}

impl App {
    pub fn new() -> App {
        let world_size = 52;
        let cube_location = Coordinate {
            x: world_size / 2,
            y: world_size / 2,
        };
        App {
            world_size,
            cube_location,
        }
    }

    pub fn move_left(&mut self) {
        if self.cube_location.x > 0 {
            self.cube_location.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.cube_location.x < self.world_size - 1 {
            self.cube_location.x += 1;
        }
    }
    pub fn move_up(&mut self) {
        if self.cube_location.y > 0 {
            self.cube_location.y -= 1;
        }
    }
    pub fn move_down(&mut self) {
        if self.cube_location.y < self.world_size - 1 {
            self.cube_location.y += 1;
        }
    }
}
