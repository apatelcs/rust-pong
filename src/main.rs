extern crate piston_window;

mod utils;

use piston_window::*;
use utils::{Paddle, Ball};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("PONG", [512; 2]).exit_on_esc(true).build().unwrap(); 
    let mut left_paddle = Paddle::new(10, 0, true);
    let mut right_paddle = Paddle::new(494, 50, false);
    let mut ball = Ball::new(200, 150);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _| {
            // draw_player(10, 0, c, g);
            // draw_player(501, 50, c, g);
            // draw_ball(200, 150, c, g);
            left_paddle.draw(&c, g);
            right_paddle.draw(&c, g);
            ball.draw(&c, g);
        });
    }
}
