extern crate piston_window;

mod utils;

use piston_window::*;
use utils::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("PONG", [512; 2]).exit_on_esc(true).build().unwrap(); 

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            draw_player(10, 0, c, g);
            draw_player(501, 50, c, g);
            draw_ball(200, 150, c, g);
        });
    }
}
