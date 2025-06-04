pub struct App {
    world_size: usize,
    cube_location: usize,
}

impl App {
    pub fn new() -> App {
        App {
            world_size: 8,
            cube_location: 3,
        }
    }
}
