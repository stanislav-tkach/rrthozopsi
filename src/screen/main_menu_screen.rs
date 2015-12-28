use screen::*;
use piston_window;

pub struct MainMenuScreen;

impl Screen for MainMenuScreen {
    fn on_input(&mut self, input: &piston_window::Input) -> InputResults {
        InputResults
    }

    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow) {
    }

    fn on_update(&mut self, args: &piston_window::UpdateArgs) {
    }
}
