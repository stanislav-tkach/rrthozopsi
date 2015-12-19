extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate find_folder;

use piston_window::Transformed;

use piston_window as pw;

mod game;
mod object;
mod events;

fn load_sprite(window: &pw::PistonWindow, name: &str) -> piston_window::Texture<gfx_device_gl::Resources> {
    // TODO: Create and use default sprite in case of failure during loading?
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let sprite = assets.join(name);
    pw::Texture::from_path(&mut *window.factory.borrow_mut(), &sprite, pw::Flip::None, &pw::TextureSettings::new()).unwrap()
}

fn main() {
    use piston_window::Event;

    let window: pw::PistonWindow = pw::WindowSettings::new("rrthozopsi", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new(load_sprite(&window, "skeleton.png"));

    for w in window {
        match w.event {
            Some(Event::Update(args)) => { game.on_update(&args); }
            Some(Event::Render(args)) => { game.on_draw(&args, &w); }
            Some(Event::Input(input)) => { game.on_input(&input); }
            _ => {}
        }
    }
}
