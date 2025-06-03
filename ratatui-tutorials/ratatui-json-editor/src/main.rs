mod app;
use crate::app::App;

fn main() {
    println!("Hello, world!");
    let mut app = App::new();
    println!("{app}");
}
