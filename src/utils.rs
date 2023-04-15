use piston_window::rectangle;
use piston_window::color::WHITE;
use piston_window::Context;
use piston_window::G2d;

const PLAYER_WIDTH: f64 = 5 as f64;
const PLAYER_HEIGHT: f64 = 30 as f64;
const BALL_WIDTH: f64 = 5 as f64;

pub fn draw_player(sx: i32, sy: i32, con: Context, g: &mut G2d) {
    rectangle(WHITE, [(sx as f64), (sy as f64), PLAYER_WIDTH, PLAYER_HEIGHT], con.transform, g);
}

pub fn draw_ball(sx: i32, sy: i32, con: Context, g: &mut G2d) {
    rectangle(WHITE, [(sx as f64), (sy as f64), BALL_WIDTH, BALL_WIDTH], con.transform, g);
}
