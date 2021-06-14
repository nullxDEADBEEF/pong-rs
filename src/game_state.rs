use ggez::audio;
use ggez::audio::SoundSource;
use ggez::graphics::{self, DrawParam};
use ggez::input::keyboard;
use ggez::{timer, Context, GameResult};

use crate::ball::Ball;
use crate::bat::Bat;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

type Pos2 = ggez::mint::Point2<f32>;

#[derive(PartialEq)]
enum States {
    Running,
    Over,
    Menu,
}

// data required to represent the game in its current state
pub struct GameState {
    dt: std::time::Duration,
    background: graphics::Image,
    bat1: Bat,
    bat2: Bat,
    ball: Ball,
    game_theme: audio::Source,
    state: States,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let game_state = Self {
            dt: std::time::Duration::new(0, 0),
            background: graphics::Image::new(ctx, "/images/table.png")?,
            bat1: Bat::new(
                Pos2 { x: -10.0, y: 150.0 },
                graphics::Image::new(ctx, "/images/bat00.png")?,
            ),
            bat2: Bat::new(
                Pos2 { x: 650.0, y: 150.0 },
                graphics::Image::new(ctx, "/images/bat10.png")?,
            ),
            ball: Ball::new(
                Pos2 {
                    x: SCREEN_WIDTH / 2.0,
                    y: SCREEN_HEIGHT / 2.0,
                },
                graphics::Image::new(ctx, "/images/ball.png")?,
            ),
            game_theme: audio::Source::new(ctx, "/music/theme.ogg")?,
            state: States::Menu,
        };

        Ok(game_state)
    }

    pub fn play_game_theme(&mut self) -> GameResult {
        self.game_theme.play_later()
    }

    fn is_menu(&self) -> bool {
        self.state == States::Menu
    }

    fn is_running(&self) -> bool {
        self.state == States::Running
    }

    fn is_gameover(&self) -> bool {
        self.state == States::Over
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        //self.play_game_theme()?;
        self.dt = timer::delta(ctx);
        self.bat1.update(
            ctx,
            self.dt.as_secs_f32(),
            keyboard::KeyCode::W,
            keyboard::KeyCode::S,
        );
        self.bat2.update(
            ctx,
            self.dt.as_secs_f32(),
            keyboard::KeyCode::Up,
            keyboard::KeyCode::Down,
        );
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
