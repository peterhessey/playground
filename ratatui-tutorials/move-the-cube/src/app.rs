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
}
