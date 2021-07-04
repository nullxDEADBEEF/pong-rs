use ggez::audio::SoundSource;
use ggez::graphics::{self, DrawParam, Font, Scale, Text};
use ggez::input::keyboard;
use ggez::nalgebra as na;
use ggez::{timer, Context, GameResult};

use crate::assets::Assets;
use crate::ball::Ball;
use crate::bat::Bat;
use crate::impact::Impact;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(PartialEq, Debug)]
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
            ctx,
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
            state: State::Menu,
        };

        Ok(game_state)
    }

    pub fn play_game_theme(&mut self) -> GameResult {
        self.assets.game_theme.set_volume(0.2);
        self.assets.game_theme.play_later()
    }

    fn get_winner_text(&self) -> Text {
        if self.bat1.score == 10 {
            return Text::new("Player 1 Won!");
        }

        Text::new("Player 2 Won!")
    }

    fn check_winnner_condition(&mut self) {
        if self.bat1.score == 10 || self.bat2.score == 10 {
            self.state = State::Over;
        }
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            State::Running => {
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
                    .collision_with_bat(ctx, &mut self.bat1, true, &self.impact_assets);
                self.ball
                    .collision_with_bat(ctx, &mut self.bat2, false, &self.impact_assets);

                self.check_winnner_condition();
            }
            State::Menu => {
                if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Space) {
                    self.state = State::Running;
                }
            }
            State::Over => {}
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match self.state {
            State::Running => {
                graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
                graphics::draw(ctx, &self.assets.background_image, DrawParam::new())?;
                self.bat1.render(ctx)?;
                self.bat2.render(ctx)?;
                self.ball.render(ctx)?;
            }
            State::Menu => {
                let mut text = Text::new("Press space to start");
                text.set_font(
                    Font::new(ctx, "/fonts/JetBrainsMono-Regular.ttf")?,
                    Scale { x: 30.0, y: 30.0 },
                );
                graphics::draw(
                    ctx,
                    &text,
                    DrawParam::new().dest(na::Point2::new(
                        SCREEN_WIDTH / 2.0 / 15.0,
                        SCREEN_HEIGHT / 2.0,
                    )),
                )?;
            }
            State::Over => {
                graphics::draw(ctx, &self.assets.gameover_image, DrawParam::new())?;
                let mut text = self.get_winner_text();
                text.set_font(
                    Font::new(ctx, "/fonts/JetBrainsMono-Regular.ttf")?,
                    Scale { x: 30.0, y: 30.0 },
                );
                graphics::draw(
                    ctx,
                    &text,
                    DrawParam::new().dest(na::Point2::new(
                        SCREEN_WIDTH / 2.0 - 25.0,
                        SCREEN_HEIGHT / 2.0 + 60.0,
                    )),
                )?;
            }
        }

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }
}
