use screen::*;
use context::Context;
use piston_window::{self, PistonWindow};

pub struct Battle;

impl Battle {
    pub fn new(_window: &PistonWindow, _context: &mut Context) -> Self {
        Battle
    }
}

impl Screen for Battle {
    fn on_input(&mut self, _input: &piston_window::Input, _: &PistonWindow, _: &mut Context) -> InputResults {
        let result = Vec::new();

        // TODO: Handle esc.

        result
    }

    fn on_draw(&mut self, _args: &piston_window::RenderArgs, window: &PistonWindow) {
        window.draw_2d(|context, graphics| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], graphics);
        });
    }

    fn on_update(&mut self, _args: &piston_window::UpdateArgs) {}
}
