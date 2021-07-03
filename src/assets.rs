use ggez::audio;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

pub struct Assets {
    pub background_image: graphics::Image,
    pub game_theme: audio::Source,
    pub ball_image: graphics::Image,
    pub bat1_default_image: graphics::Image,
    pub bat2_default_image: graphics::Image,
}

impl Assets {
    pub fn load_initial_assets(ctx: &mut Context) -> GameResult<Assets> {
        let background_image = graphics::Image::new(ctx, "/images/table.png")?;
        let game_theme = audio::Source::new(ctx, "/music/theme.ogg")?;
        let ball_image = graphics::Image::new(ctx, "/images/ball.png")?;
        let bat1_default_image = graphics::Image::new(ctx, "/images/bat00.png")?;
        let bat2_default_image = graphics::Image::new(ctx, "/images/bat10.png")?;
        Ok(Self {
            background_image,
            game_theme,
            ball_image,
            bat1_default_image,
            bat2_default_image,
        })
    }
}
