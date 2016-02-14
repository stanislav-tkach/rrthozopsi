extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;
#[macro_use]
extern crate conrod;

mod game;
mod screen;
mod object;
mod ui_utils;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
