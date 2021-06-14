mod ball;
mod bat;
mod game_state;

use std::path;

use ggez::*;

use game_state::GameState;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 480.0;

fn main() -> GameResult {
    let asset_path = path::PathBuf::from("./assets");

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Pong", "nullxDEADBEEF")
        .add_resource_path(asset_path)
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();
    ggez::graphics::set_window_title(ctx, "Pong");

    let mut game_state = GameState::new(ctx)?;

    event::run(ctx, event_loop, &mut game_state)
}
