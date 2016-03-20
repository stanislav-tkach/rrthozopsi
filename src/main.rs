extern crate piston_window;
#[macro_use]
extern crate conrod;

mod context;
mod game;
mod screen;
mod ui;
mod resource_manager;
mod graphics;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
