#[cfg(feature = "buddy-alloc")]
mod alloc;
mod game;
mod input;
mod palette;
mod rng;
mod wasm4;

use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    palette::set_palette([0xfff6d3, 0xf9a875, 0xeb6b6f, 0x7c3f58]);
}

#[no_mangle]
fn update() {
    GAME.lock().expect("game_update").update();
}
