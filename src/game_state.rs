use ggez::{Context, GameResult, timer};
use ggez::graphics::{self, DrawParam};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ball::Ball;
use crate::bat::Bat;

type Pos2 = ggez::mint::Point2<f32>;

enum States {
    GameRunning,
    GameOver,
    GameMenu,
}

// data required to represent the game in its current state
pub struct GameState {
    dt: std::time::Duration,
    background: graphics::Image,
    bat1: Bat,
    bat2: Bat,
    ball: Ball,
    state: States,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let game_state = Self {
            dt: std::time::Duration::new(0, 0),
            background: graphics::Image::new(ctx, "/images/table.png")?,
            bat1: Bat::new(Pos2 { x: 50.0, y: 50.0 }, &graphics::Image::new(ctx, "/images/bat00.png")?),
            bat2: Bat::new(Pos2 { x: 400.0, y: 50.0 }, &graphics::Image::new(ctx, "/images/bat10.png")?),
            ball: Ball::new(Pos2 { x: SCREEN_WIDTH / 2.0, y: SCREEN_HEIGHT / 2.0}, &graphics::Image::new(ctx, "/images/ball.png")?),
            state: States::GameMenu,
        };

        Ok(game_state)
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx);
        self.bat1.update(ctx);
        self.bat2.update(ctx);
        self.ball.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        graphics::draw(ctx, &self.background, DrawParam::new())?;
        self.bat1.render(ctx)?;
        self.bat2.render(ctx)?;
        self.ball.render(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}