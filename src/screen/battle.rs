use screen::*;
use context::Context;
use piston_window::{self, PistonWindow};

pub struct Battle {
    main_menu: bool,
}

impl Battle {
    pub fn new(_window: &PistonWindow, _context: &mut Context) -> Self {
        Battle { main_menu: false }
    }
}

impl Screen for Battle {
    fn on_input(&mut self, input: &piston_window::Input, _: &PistonWindow, _: &mut Context) -> InputResults {
        use piston_window::{Input, Button, Key};

        let result = Vec::new();

        match input {
            &Input::Press(button) => {
                match button {
                    Button::Keyboard(Key::Escape) => {
                        self.main_menu = true;
                    }
                    _ => {}
                }
            }
            &Input::Release(button) => {
                match button {
                    Button::Keyboard(Key::Escape) => {
                        self.main_menu = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        result
    }

    fn on_draw(&mut self, _args: &piston_window::RenderArgs, window: &PistonWindow) {
        window.draw_2d(|_context, graphics| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], graphics);
        });
    }

    fn on_update(&mut self, _args: &piston_window::UpdateArgs) {}
}
