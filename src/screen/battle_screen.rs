use screen::*;

use ::object;

use piston_window;

pub struct BattleScreen {
    player: object::Object,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

// TODO: Remove
fn load_sprite(window: &piston_window::PistonWindow, name: &str) -> piston_window::Texture<gfx_device_gl::Resources> {
    // TODO: Create and use default sprite in case of failure during loading?
    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
    let sprite = assets.join(name);
    piston_window::Texture::from_path(&mut *window.factory.borrow_mut(), &sprite, piston_window::Flip::None, &piston_window::TextureSettings::new()).unwrap()
}

impl BattleScreen {
    pub fn new() -> BattleScreen {
        BattleScreen{ player: load_sprite(&window, "skeleton.png"), }
    }
}

impl Screen for BattleScreen {
    fn on_input(&mut self, input: &piston_window::Input) -> InputResults {
        use piston_window::{Input, Button, Key};

        let result = Vec::new();

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

        result
    }

    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow) {
	window.draw_2d(|context, graphics| {
	    piston_window::clear([0.0, 0.0, 0.0, 1.0], graphics);

            let center = context.transform.trans((args.width / 2) as f64, (args.height / 2) as f64);
            self.player.render(graphics, &center);
	});
    }

    fn on_update(&mut self, args: &piston_window::UpdateArgs) {
	if self.up   { self.player.mov(0.0, -150.0 * args.dt); }
        if self.down { self.player.mov(0.0, 150.0 * args.dt); }
        if self.left { self.player.mov(-150.0 * args.dt, 0.0); }
        if self.right { self.player.mov(150.0 * args.dt, 0.0); }
    }
}
