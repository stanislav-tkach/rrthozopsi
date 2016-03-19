extern crate piston_window;
extern crate gfx_device_gl;
#[macro_use]
extern crate conrod;

mod context;
mod game;
mod screen;
mod ui;
mod resource_manager;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
