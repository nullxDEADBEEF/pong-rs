use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam};

type Pos2 = ggez::mint::Point2<f32>;

pub struct Bat {
    position: Pos2,
    sprite: graphics::Image,
    collider: graphics::Rect,
}

impl Bat {
    pub fn new(pos: Pos2, sprite: &graphics::Image) -> Self {
        Self {
            position: pos,
            sprite: sprite.clone(),
            collider: graphics::Rect::new(pos.x, pos.y, sprite.width() as f32, sprite.height() as f32),
        }
    }

    pub fn update(&self, ctx: &mut Context) {
        
    }

    pub fn render(&self, ctx: &mut Context) -> GameResult {
        Ok(graphics::draw(ctx, &self.sprite, DrawParam::new().dest(self.position))?)
    }
}