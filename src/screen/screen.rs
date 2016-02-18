use piston_window::{self, PistonWindow};

pub enum InputResult {
    PushScreen(Box<Screen>),
    PopScreen,
}

pub type InputResults = Vec<InputResult>;

pub struct Context {
    pub assets_path: ::std::path::PathBuf,
}

pub trait Screen {
    fn on_input(&mut self, input: &piston_window::Input, window: &PistonWindow, context: &mut Context) -> InputResults;
    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &PistonWindow);
    fn on_update(&mut self, args: &piston_window::UpdateArgs);
}
