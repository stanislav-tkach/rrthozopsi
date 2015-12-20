extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate find_folder;

use piston_window::Transformed;

use piston_window as pw;

mod game;
mod screen;
mod object;
mod events;

fn main() {
    let mut game = game::Game::new(load_sprite(&window, "skeleton.png"));
    game.run();


    use piston_window::Event;

    let window: pw::PistonWindow = pw::WindowSettings::new("rrthozopsi", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    for w in window {
        match w.event {
            Some(Event::Update(args)) => { game.on_update(&args); }
            Some(Event::Render(args)) => { game.on_draw(&args, &w); }
            Some(Event::Input(input)) => { game.on_input(&input); }
            _ => {}
        }
    }
}
