extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate conrod;

mod game;
mod screen;
mod object;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
