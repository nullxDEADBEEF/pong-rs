use ggez::graphics::{self, DrawParam, Rect};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::SCREEN_HEIGHT;

pub struct Bat {
    pub position: na::Point2<f32>,
    pub sprite: graphics::Image,
    pub collider: graphics::Rect,
}

impl Bat {
    pub const PLAYER_SPEED: f32 = 500.0;

    pub fn new(pos: na::Point2<f32>, sprite: graphics::Image) -> Self {
        Self {
            position: pos,
            sprite: sprite.clone(),
            collider: Rect::new(pos.x + sprite.width() as f32, pos.y, 20.0, 125.0),
        }
    }

    fn move_up(&mut self, dt: f32) {
        if self.position.y <= 0.0 {
            self.position.y = 0.0;
        } else {
            self.position.y -= Bat::PLAYER_SPEED * dt;
        }
    }

    fn move_down(&mut self, dt: f32) {
        if self.position.y >= SCREEN_HEIGHT - self.sprite.height() as f32 {
            self.position.y = SCREEN_HEIGHT - self.sprite.height() as f32;
        } else {
            self.position.y += Bat::PLAYER_SPEED * dt;
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        dt: f32,
        up_key: keyboard::KeyCode,
        down_key: keyboard::KeyCode,
    ) {
        if keyboard::is_key_pressed(ctx, up_key) {
            self.move_up(dt);
        }
        if keyboard::is_key_pressed(ctx, down_key) {
            self.move_down(dt);
        }
        self.collider.x = self.position.x;
        self.collider.y = self.position.y;
        self.collider.move_to(na::Point2::new(
            self.position.x + 70.0,
            self.position.y + 15.0,
        ));
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, DrawParam::new().dest(self.position))
    }
}
