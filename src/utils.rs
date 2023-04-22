use piston_window::rectangle;
use piston_window::ellipse;
use piston_window::color::WHITE;
use piston_window::Context;
use piston_window::G2d;

const PADDLE_WIDTH: f64 = 8 as f64;
const PADDLE_HEIGHT: f64 = 48 as f64;
const BALL_RAD: f64 = 8 as f64;

// pub fn draw_player(sx: i32, sy: i32, con: Context, g: &mut G2d) {
//     rectangle(WHITE, [(sx as f64), (sy as f64), PLAYER_WIDTH, PLAYER_HEIGHT], con.transform, g);
// }

// pub fn draw_ball(sx: i32, sy: i32, con: Context, g: &mut G2d) {
//     rectangle(WHITE, [(sx as f64), (sy as f64), BALL_WIDTH, BALL_WIDTH], con.transform, g);
// }

pub struct Paddle {
    pub x: i32,
    pub y: i32,
    pub is_player: bool,
}

impl Paddle {
    pub fn new(x: i32, y: i32, is_player: bool) -> Paddle {
        Paddle {
            x: x,
            y: y,
            is_player: is_player,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        rectangle(WHITE, [(self.x as f64), (self.y as f64), PADDLE_WIDTH, PADDLE_HEIGHT], con.transform, g);
    }
}

pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub x_vel: f64,
    pub y_vel: f64,
}

impl Ball {
    pub fn new(x: i32, y: i32) -> Ball {
        Ball {
            x: x,
            y: y,
            x_vel: 1.0,
            y_vel: 0.0,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        ellipse(WHITE, [(self.x as f64), (self.y as f64), BALL_RAD, BALL_RAD], con.transform, g);
    }
}