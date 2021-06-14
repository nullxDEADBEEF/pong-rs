use ggez::graphics::{self, DrawParam};
use ggez::input::keyboard;
use ggez::{Context, GameResult};

use crate::SCREEN_HEIGHT;

type Pos2 = ggez::mint::Point2<f32>;

pub struct Bat {
    position: Pos2,
    sprite: graphics::Image,
    collider: graphics::Rect,
}

impl Bat {
    const PLAYER_SPEED: f32 = 500.0;

    pub fn new(pos: Pos2, sprite: graphics::Image) -> Self {
        Self {
            position: pos,
            sprite: sprite.clone(),
            collider: graphics::Rect::new(
                pos.x,
                pos.y,
                sprite.width() as f32,
                sprite.height() as f32,
            ),
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
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, DrawParam::new().dest(self.position))
    }
}
