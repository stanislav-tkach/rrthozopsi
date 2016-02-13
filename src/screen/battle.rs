use screen::*;
use object;
use piston_window::{self, Transformed};
use gfx_device_gl;

pub struct Battle {
    player: object::Object,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

fn load_sprite(window: &piston_window::PistonWindow,
               assets_path: &::std::path::Path,
               name: &str)
               -> piston_window::Texture<gfx_device_gl::Resources> {
    // TODO: Create and use default sprite in case of failure during loading?
    let sprite = assets_path.join(name);
    piston_window::Texture::from_path(&mut *window.factory.borrow_mut(),
                                      &sprite,
                                      piston_window::Flip::None,
                                      &piston_window::TextureSettings::new())
        .unwrap()
}

impl Battle {
    pub fn new(window: &piston_window::PistonWindow, context: &mut Context) -> Self {
        Battle {
            player: object::Object::new(Some(load_sprite(&window, &context.assets_path, "skeleton.png"))),
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

impl Screen for Battle {
    fn on_input(&mut self,
                input: &piston_window::Input,
                window: &piston_window::PistonWindow,
                context: &mut Context)
                -> InputResults {
        use piston_window::{Input, Button, Key};

        let result = Vec::new();

        match input {
            &Input::Press(button) => {
                match button {
                    Button::Keyboard(Key::Up) => {
                        self.up = true;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down = true;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left = true;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right = true;
                    }
                    _ => {}
                }
            }
            &Input::Release(button) => {
                match button {
                    Button::Keyboard(Key::Up) => {
                        self.up = false;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down = false;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left = false;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right = false;
                    }
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
        if self.up {
            self.player.mov(0.0, -150.0 * args.dt);
        }
        if self.down {
            self.player.mov(0.0, 150.0 * args.dt);
        }
        if self.left {
            self.player.mov(-150.0 * args.dt, 0.0);
        }
        if self.right {
            self.player.mov(150.0 * args.dt, 0.0);
        }
    }
}
