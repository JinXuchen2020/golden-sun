mod game;

use macroquad::prelude::next_frame;

#[macroquad::main("Golden Sun - Rust Edition")]
async fn main() {
    let mut ctx = game::GameCtx::new();
    loop {
        ctx.step();
        next_frame().await;
    }
}
