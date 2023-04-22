use piston_window::{Context, G2d};
use crate::utils::{Ball, Paddle, WINDOW_WIDTH, PADDLE_WIDTH, PADDLE_HEIGHT};

pub struct Game {
    l_paddle: Paddle,
    r_paddle: Paddle,
    ball: Ball,
    score: i32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            l_paddle: Paddle::new(10, 255, true),
            r_paddle: Paddle::new(494, 255, false),
            ball: Ball::new(255, 255),
            score: 0,
        }
    }

    pub fn update(&mut self, con: &Context, g: &mut G2d, key: char, is_held: bool) {
        self.l_paddle.update(key, is_held, self.ball.y);
        self.r_paddle.update(key, is_held, self.ball.y);
        self.ball.update();
        self.l_paddle.draw(con, g);
        self.r_paddle.draw(con, g);
        self.ball.draw(con, g);
        self.state_check();
    }

    pub fn state_check(&mut self) {
        if self.ball.x <= 0 {
            self.score -= 1;
            println!("YOU LOST! Score: {}", self.score);
        } else if self.ball.x >= (WINDOW_WIDTH - PADDLE_WIDTH) as i32 {
            self.score += 1;
            println!("YOU WON! Score: {}", self.score);
        } else if self.ball.x <= 10 + (PADDLE_WIDTH as i32) {
            if (self.ball.y <= self.l_paddle.y + (PADDLE_HEIGHT as i32)) && (self.ball.y >= self.l_paddle.y) {
                self.ball.x_vel *= -1;
            }
        } else if self.ball.x >= 494 {
            if (self.ball.y <= self.r_paddle.y + (PADDLE_HEIGHT as i32)) && (self.ball.y >= self.r_paddle.y) {
                self.ball.x_vel *= -1;
            }
        }
    }
}