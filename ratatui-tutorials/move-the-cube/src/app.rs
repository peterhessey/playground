pub struct App {
    pub world_size: u16,
    pub cube_location: u16,
}

impl App {
    pub fn new() -> App {
        App {
            world_size: 20,
            cube_location: 10,
        }
    }

    pub fn move_left(&mut self) {
        if self.cube_location > 0 {
            self.cube_location -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.cube_location < self.world_size - 1 {
            self.cube_location += 1;
        }
    }
}
