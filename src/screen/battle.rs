use screen::*;
use context::Context;
use resource_manager::Textures;
use graphics::Texture;
use piston_window::{self, PistonWindow, ImageSize, Transformed};

pub struct Battle {
    empty_tile: Texture,
}

impl Battle {
    pub fn new(window: &PistonWindow, context: &mut Context) -> Self {
        Battle { empty_tile: context.resource_manager.load_texture(window, Textures::EmptyTile) }
    }
}

impl Screen for Battle {
    fn on_input(&mut self, input: &piston_window::Input, window: &PistonWindow, context: &mut Context) -> InputResults {
        use piston_window::{Input, Button, Key};

        let mut result = Vec::new();

        match input {
            &Input::Press(button) => {
                match button {
                    Button::Keyboard(Key::Escape) => {
                        result.push(InputResult::PushScreen(Box::new(MainMenu::new(&window, context))));
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        result
    }

    fn on_draw(&mut self, _args: &piston_window::RenderArgs, window: &PistonWindow) {
        window.draw_2d(|context, graphics| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], graphics);

            // Draw grid.
            let tiles_count = 4;
            let (height, width) = self.empty_tile.get_size();
            for x in 0..tiles_count {
                for y in 0..tiles_count {
                    // TODO: draw_many?
                    piston_window::image(&self.empty_tile, context.transform.trans((x * height + 5) as f64, (y * width + 5) as f64), graphics);
                }
            }
        });
    }

    fn on_update(&mut self, _args: &piston_window::UpdateArgs) {}
}
