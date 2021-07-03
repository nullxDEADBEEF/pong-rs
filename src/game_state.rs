use ggez::audio::SoundSource;
use ggez::graphics::{self, DrawParam};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::{timer, Context, GameResult};

use crate::assets::Assets;
use crate::ball::Ball;
use crate::bat::Bat;
use crate::impact::Impact;
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
    assets: Assets,
    impact_assets: Vec<graphics::Image>,
    bat1: Bat,
    bat2: Bat,
    ball: Ball,
    state: State,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let assets = Assets::load_initial_assets(ctx)?;
        let impact_assets = Impact::load_assets(ctx)?;

        let bat1 = Bat::new(na::Point2::new(0.0, 150.0), &assets.bat1_default_image);
        let bat2 = Bat::new(na::Point2::new(650.0, 150.0), &assets.bat2_default_image);
        let ball = Ball::new(
            na::Point2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            &assets.ball_image,
        );

        let game_state = Self {
            dt: 0.0,
            assets,
            impact_assets,
            bat1,
            bat2,
            ball,
            state: State::Running,
        };

        Ok(game_state)
    }

    pub fn play_game_theme(&mut self) -> GameResult {
        self.assets.game_theme.set_volume(0.2);
        self.assets.game_theme.play_later()
    }

    fn is_running_state(&self) -> bool {
        self.state == State::Running
    }

    fn is_menu_state(&self) -> bool {
        self.state == State::Menu
    }

    fn is_gameover_state(&self) -> bool {
        self.state == State::Over
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.is_running_state() {
            if self.assets.game_theme.stopped() {
                self.play_game_theme()?;
            }
            self.dt = timer::delta(ctx).as_secs_f32();
            self.bat1
                .update(ctx, self.dt, keyboard::KeyCode::W, keyboard::KeyCode::S);
            self.bat2
                .update(ctx, self.dt, keyboard::KeyCode::Up, keyboard::KeyCode::Down);
            self.ball.update(ctx, self.dt);
            self.ball
                .collision_with_bat(ctx, &self.bat1, true, &self.impact_assets);
            self.ball
                .collision_with_bat(ctx, &self.bat2, false, &self.impact_assets);
        }
        if self.is_menu_state() {}
        if self.is_gameover_state() {}
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.is_running_state() {
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
            graphics::draw(ctx, &self.assets.background_image, DrawParam::new())?;
            self.bat1.render(ctx)?;
            self.bat2.render(ctx)?;
            self.ball.render(ctx)?;
        }
        if self.is_menu_state() {}
        if self.is_gameover_state() {}

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}
