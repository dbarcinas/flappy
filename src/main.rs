mod mode;
mod obstacle;
mod player;
mod state;

use bracket_lib::terminal::{main_loop, BError, BTermBuilder};

use crate::state::State;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Flappy Crab")
        .build()?;

    main_loop(ctx, State::new())
}
