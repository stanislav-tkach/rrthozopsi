extern crate piston_window;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate find_folder;

use piston_window::Transformed;

use piston_window as pw;

mod object;
use object::Object;

struct Game {
    player: Object,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Game {
    fn new(sprite: piston_window::Texture<gfx_device_gl::Resources>) -> Game {
        Game { player: Object::new(Some(sprite)), up: false, down: false, left: false, right: false }
    }

    fn on_update(&mut self, args: &pw::UpdateArgs) {
        if self.up   { self.player.mov(0.0, -150.0 * args.dt); }
        if self.down { self.player.mov(0.0, 150.0 * args.dt); }
        if self.left { self.player.mov(-150.0 * args.dt, 0.0); }
        if self.right { self.player.mov(150.0 * args.dt, 0.0); }
    }

    fn on_draw(&mut self, args: &pw::RenderArgs, window: &pw::PistonWindow) {
        window.draw_2d(|context, graphics| {
            pw::clear([0.0, 0.0, 0.0, 1.0], graphics);

            let center = context.transform.trans((args.width / 2) as f64, (args.height / 2) as f64);
            self.player.render(graphics, &center);
        });
    }

    fn on_input(&mut self, input: &pw::Input) {
        use piston_window::{Input, Button, Key};

        match input {
            &Input::Press(button) => {
                match button {
                    Button::Keyboard(Key::Up) => { self.up = true; }
                    Button::Keyboard(Key::Down) => { self.down = true; }
                    Button::Keyboard(Key::Left) => { self.left = true; }
                    Button::Keyboard(Key::Right) => { self.right = true; }
                    _ => {}
                }
            }
            &Input::Release(button) => {
                match button {
                    Button::Keyboard(Key::Up) => { self.up = false; }
                    Button::Keyboard(Key::Down) => { self.down = false; }
                    Button::Keyboard(Key::Left) => { self.left = false; }
                    Button::Keyboard(Key::Right) => { self.right = false; }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

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

    let mut game = Game::new(load_sprite(&window, "skeleton.png"));

    for w in window {
        match w.event {
            Some(Event::Update(args)) => { game.on_update(&args); }
            Some(Event::Render(args)) => { game.on_draw(&args, &w); }
            Some(Event::Input(input)) => { game.on_input(&input); }
            _ => {}
        }
    }
}
