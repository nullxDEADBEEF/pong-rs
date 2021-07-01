use std::time::Duration;

use ggez::audio;
use ggez::audio::SoundSource;
use ggez::graphics::{self, DrawParam};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::{timer, Context, GameResult};

use crate::ball::Ball;
use crate::bat::Bat;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(PartialEq)]
enum State {
    Running,
    Over,
    Menu,
}

// data required to represent the game in its current state
pub struct GameState {
    dt: f32,
    background: graphics::Image,
    bat1: Bat,
    bat2: Bat,
    ball: Ball,
    game_theme: audio::Source,
    state: State,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let game_state = Self {
            dt: 0.0,
            background: graphics::Image::new(ctx, "/images/table.png")?,
            bat1: Bat::new(
                na::Point2::new(0.0, 150.0),
                graphics::Image::new(ctx, "/images/bat00.png")?,
            ),
            bat2: Bat::new(
                na::Point2::new(650.0, 150.0),
                graphics::Image::new(ctx, "/images/bat10.png")?,
            ),
            ball: Ball::new(
                na::Point2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
                graphics::Image::new(ctx, "/images/ball.png")?,
            ),
            game_theme: audio::Source::new(ctx, "/music/theme.ogg")?,
            state: State::Running,
        };

        Ok(game_state)
    }

    pub fn play_game_theme(&mut self) -> GameResult {
        self.game_theme.set_volume(0.2);
        self.game_theme.play_later()
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            State::Running => {
                // TODO: uncomment
                //self.play_game_theme()?;
                self.dt = timer::delta(ctx).as_secs_f32();
                self.bat1
                    .update(ctx, self.dt, keyboard::KeyCode::W, keyboard::KeyCode::S);
                self.bat2
                    .update(ctx, self.dt, keyboard::KeyCode::Up, keyboard::KeyCode::Down);
                self.ball.update(ctx, self.dt);
                self.ball.collision_with_bat(&self.bat1, true);
                self.ball.collision_with_bat(&self.bat2, false);
            }
            State::Menu => {}
            State::Over => {}
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            State::Running => {
                graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
                graphics::draw(ctx, &self.background, DrawParam::new())?;
                self.bat1.render(ctx)?;
                self.bat2.render(ctx)?;
                self.ball.render(ctx)?;
                graphics::present(ctx)?;
            }
            State::Menu => {}
            State::Over => {}
        }

        timer::sleep(Duration::from_millis(1));
        Ok(())
    }
}
