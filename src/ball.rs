use ggez::nalgebra as na;
use ggez::timer;
use ggez::{
    graphics::{self, DrawParam},
    Context, GameResult,
};

use crate::bat::Bat;
use crate::SCREEN_HEIGHT;

pub struct Ball {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
    speed: f32,
    sprite: graphics::Image,
    collider: graphics::Rect,
}

impl Ball {
    const MAX_VELOCITY: f32 = 250.0;
    const MIN_VELOCITY: f32 = 150.0;
    const BALL_SPEED_UPDATE_RATE: usize = 100;

    pub fn new(pos: na::Point2<f32>, sprite: graphics::Image) -> Self {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let mut velocity_x = rng.gen_range(Ball::MIN_VELOCITY..Ball::MAX_VELOCITY);
        let mut velocity_y = rng.gen_range(Ball::MIN_VELOCITY..Ball::MAX_VELOCITY);

        // randomize where the ball goes on start
        if rng.gen::<bool>() {
            velocity_x *= -1.0;
        }
        if rng.gen::<bool>() {
            velocity_y *= -1.0;
        }

        Self {
            position: pos,
            velocity: na::Vector2::<f32>::new(velocity_x, velocity_y),
            speed: 1.0,
            sprite,
            collider: graphics::Rect::new(pos.x, pos.y, 24.0, 24.0),
        }
    }

    pub fn collision_with_bat(&mut self, bat: &Bat, is_bat1: bool) {
        if self.collider.overlaps(&bat.collider) {
            if is_bat1 {
                self.velocity.x = self.velocity.x.abs();
            } else {
                self.velocity.x = -self.velocity.x.abs();
            }
        }
    }

    pub fn update(&mut self, ctx: &mut Context, dt: f32) {
        self.position.x += self.velocity.x * self.speed * dt;
        self.position.y += self.velocity.y * self.speed * dt;
        self.collider.x = self.position.x;
        self.collider.y = self.position.y;

        if self.velocity.y < 0.0 && self.collider.top() < 0.0
            || self.velocity.y > 0.0 && self.collider.bottom() > SCREEN_HEIGHT
        {
            self.velocity.y *= -1.0;
        }
        if timer::ticks(ctx) % Ball::BALL_SPEED_UPDATE_RATE == 0 {
            self.speed += 0.1;
        }
        if self.speed >= 3.0 {
            self.speed = 3.0;
        }
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, DrawParam::new().dest(self.position))
    }
}
