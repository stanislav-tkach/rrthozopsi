extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;

mod game;
mod screen;
mod object;
mod events;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
