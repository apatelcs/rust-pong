use piston_window::rectangle;
use piston_window::ellipse;
use piston_window::color::WHITE;
use piston_window::Context;
use piston_window::G2d;

pub const PADDLE_WIDTH: f64 = 8 as f64;
pub const PADDLE_HEIGHT: f64 = 48 as f64;
const BALL_RAD: f64 = 8 as f64;
pub const WINDOW_WIDTH: f64 = 512 as f64;
pub const WINDOW_HEIGHT: f64 = 512 as f64;

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

    pub fn update(&mut self, key: char, is_held: bool, ball_y: i32) {
        if self.is_player {
            if (key == 'u') && (is_held) {
                self.y -= 8;
            } else if (key == 'd') && (is_held) {
                self.y += 8;
            }
        } else {
            if self.y > ((PADDLE_HEIGHT / 4.0) as i32) + ball_y {
                self.y -= 10;
            } else if self.y < ball_y - ((PADDLE_HEIGHT / 4.0) as i32) {
                self.y += 10;
            }
        }
    }
}

pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub x_vel: i32,
    pub y_vel: i32,
}

impl Ball {
    pub fn new(x: i32, y: i32) -> Ball {
        Ball {
            x: x,
            y: y,
            x_vel: 1,
            y_vel: 2,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        ellipse(WHITE, [(self.x as f64), (self.y as f64), BALL_RAD, BALL_RAD], con.transform, g);
    }

    pub fn update(&mut self) {
        if self.y <= 0 || self.y >= (WINDOW_HEIGHT as i32) {
            self.y_vel *= -1;
        }

        if self.x <= 10 || self.x >= (WINDOW_WIDTH as i32) {
            self.y = (WINDOW_HEIGHT / 2.0) as i32;
            self.x = (WINDOW_WIDTH / 2.0) as i32;
            self.x_vel *= -1;
        }

        self.x += self.x_vel;
        self.y += self.y_vel;
    }
}