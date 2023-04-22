extern crate piston_window;

mod utils;
mod game;

use piston_window::*;
use utils::{WINDOW_WIDTH, WINDOW_HEIGHT};
use game::Game;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("PONG", [WINDOW_WIDTH, WINDOW_HEIGHT]).exit_on_esc(true).build().unwrap(); 
    // let mut left_paddle = Paddle::new(10, 0, true);
    // let mut right_paddle = Paddle::new(494, 50, false);
    // let mut ball = Ball::new(200, 150);
    let mut game = Game::new();
    let mut is_key_held = false;
    let mut which_key = 'n'; // Options are n for None, u for Up, and d for Down
    // Consider changing above to an enum?

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            is_key_held = true;
            match key {
                Key::Up => {
                    which_key = 'u';
                },
                Key::Down => {
                    which_key = 'd';
                },
                _ => {},
            }
        }

        if let Some(Button::Keyboard(_key)) = event.release_args() {
            is_key_held = false;
            which_key = 'n';
        }

        window.draw_2d(&event, |c, g, _| {
            // left_paddle.draw(&c, g);
            // right_paddle.draw(&c, g);
            // ball.draw(&c, g);
            clear(color::BLACK, g);
            game.update(&c, g, which_key, is_key_held);
        });
    }
}
