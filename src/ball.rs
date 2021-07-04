use ggez::audio;
use ggez::audio::SoundSource;
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{
    graphics::{self, DrawParam},
    Context, GameResult,
};

use crate::bat::Bat;
use crate::impact::Impact;
use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;

pub struct Ball {
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
    speed: f32,
    sprite: graphics::Image,
    collider: graphics::Rect,
    impact: Option<Impact>,
    goal_sound: audio::Source,
}

impl Ball {
    const MAX_VELOCITY: f32 = 250.0;
    const MIN_VELOCITY: f32 = 150.0;
    const BALL_SPEED_UPDATE_RATE: usize = 100;

    pub fn new(ctx: &mut Context, pos: na::Point2<f32>, sprite: &graphics::Image) -> Self {
        Self {
            position: pos,
            velocity: Ball::random_start_vec(),
            speed: 1.0,
            sprite: sprite.clone(),
            collider: graphics::Rect::new(pos.x, pos.y, 24.0, 24.0),
            impact: None,
            goal_sound: audio::Source::new(ctx, "/sounds/score_goal0.ogg").unwrap(),
        }
    }

    fn random_start_vec() -> na::Vector2<f32> {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let mut velocity_x = rng.gen_range(Ball::MIN_VELOCITY..Ball::MAX_VELOCITY);
        let mut velocity_y = rng.gen_range(Ball::MIN_VELOCITY..Ball::MAX_VELOCITY);

        // randomize where the ball goes on start
        if rng.gen::<bool>() {
            velocity_x *= -1.0;
        }
        if rng.gen::<bool>() {
            velocity_y *= -1.0;
        }

        na::Vector2::<f32>::new(velocity_x, velocity_y)
    }

    fn play_goal_sound(&mut self) {
        self.goal_sound.set_volume(0.2);
        self.goal_sound.play().expect("could not play goal sound");
    }

    fn reset(&mut self) {
        self.position = na::Point2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0);
        self.speed = 1.0;
        self.velocity = Ball::random_start_vec();
    }

    pub fn collision_with_bat(
        &mut self,
        ctx: &mut Context,
        bat: &mut Bat,
        is_bat1: bool,
        impact_assets: &[graphics::Image],
    ) {
        if self.collider.overlaps(&bat.collider) {
            let impact_position = na::Point2::new(self.position.x - 10.0, self.position.y);
            self.impact = Some(
                Impact::new(
                    ctx,
                    na::Point2::new(impact_position.x, impact_position.y),
                    impact_assets,
                )
                .expect("could not create impact"),
            );
            self.impact
                .as_mut()
                .unwrap()
                .play_hit_sound()
                .expect("could not play hit sound");
            if is_bat1 {
                self.velocity.x = self.velocity.x.abs();
            } else {
                self.velocity.x = -self.velocity.x.abs();
            }
        }

        if self.impact.is_some() {
            self.impact.as_mut().unwrap().update();
        }

        if self.position.x < 0.0 && !is_bat1 {
            bat.score += 1;
            self.reset();
            self.play_goal_sound();
        } else if self.position.x > SCREEN_WIDTH && is_bat1 {
            self.reset();
            self.play_goal_sound();
            bat.score += 1;
        }
    }

    pub fn update(&mut self, ctx: &mut Context, dt: f32) {
        self.position.x += (self.velocity.x * self.speed) * dt;
        self.position.y += (self.velocity.y * self.speed) * dt;
        self.collider.x = self.position.x;
        self.collider.y = self.position.y;

        if self.velocity.y < 0.0 && self.collider.top() < 0.0
            || self.velocity.y > 0.0 && self.collider.bottom() > SCREEN_HEIGHT
        {
            self.velocity.y *= -1.0;
        }
        if timer::ticks(ctx) % Ball::BALL_SPEED_UPDATE_RATE == 0 {
            self.speed += 0.1;
        }
        if self.speed >= 3.0 {
            self.speed = 3.0;
        }
    }

    pub fn render(&mut self, ctx: &mut Context) -> GameResult {
        graphics::draw(ctx, &self.sprite, DrawParam::new().dest(self.position))?;
        if self.impact.is_some() {
            self.impact.as_mut().unwrap().render(ctx)?;
        }
        Ok(())
    }
}
