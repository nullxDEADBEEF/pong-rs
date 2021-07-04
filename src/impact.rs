use ggez::audio;
use ggez::audio::SoundSource;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::nalgebra as na;
use ggez::Context;
use ggez::GameResult;

pub struct Impact {
    position: na::Point2<f32>,
    sprite: graphics::Image,
    sprites: Vec<graphics::Image>,
    hit_sound: audio::Source,
    current_frame: i32,
    frame_rate: i32,
}

impl Impact {
    const FRAME_SPEED: i32 = 6;

    pub fn load_assets(ctx: &mut Context) -> GameResult<Vec<graphics::Image>> {
        Ok(vec![
            graphics::Image::new(ctx, "/images/impact0.png")?,
            graphics::Image::new(ctx, "/images/impact1.png")?,
            graphics::Image::new(ctx, "/images/impact2.png")?,
            graphics::Image::new(ctx, "/images/impact3.png")?,
            graphics::Image::new(ctx, "/images/impact4.png")?,
            graphics::Image::new(ctx, "/images/blank.png")?,
        ])
    }

    pub fn new(
        ctx: &mut Context,
        pos: na::Point2<f32>,
        sprites: &[graphics::Image],
    ) -> GameResult<Self> {
        Ok(Self {
            position: pos,
            sprite: sprites[0].clone(),
            sprites: sprites.to_owned(),
            hit_sound: audio::Source::new(ctx, "/sounds/bounce0.ogg")?,
            current_frame: 0,
            frame_rate: 6,
        })
    }

    fn animate(&mut self) {
        if self.frame_rate == (Impact::FRAME_SPEED - 1) {
            self.current_frame += 1;
            if self.current_frame as usize >= self.sprites.len() {
                return;
            }
            self.sprite = self
                .sprites
                .get(self.current_frame as usize)
                .unwrap()
                .clone();
        }
        self.frame_rate = (self.frame_rate + 1) % Impact::FRAME_SPEED;
    }

    pub fn play_hit_sound(&mut self) -> GameResult {
        self.hit_sound.play()
    }

    pub fn update(&mut self) {
        self.animate()
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, DrawParam::new().dest(self.position))
    }
}
