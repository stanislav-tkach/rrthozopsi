use screen::*;
use context::Context;
use std::path::Path;
use piston_window::{self, PistonWindow, Transformed};
use gfx_device_gl;

pub struct Battle;

impl Battle {
    pub fn new(window: &PistonWindow, context: &mut Context) -> Self {
        Battle
    }
}

impl Screen for Battle {
    fn on_input(&mut self, input: &piston_window::Input, _: &PistonWindow, _: &mut Context) -> InputResults {
        let result = Vec::new();

        // TODO: Handle esc.

        result
    }

    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &PistonWindow) {
        window.draw_2d(|context, graphics| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], graphics);
        });
    }

    fn on_update(&mut self, args: &piston_window::UpdateArgs) {}
}
