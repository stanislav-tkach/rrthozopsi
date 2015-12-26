use piston_window;

enum InputResult {
    PushScreen(Box<Screen>),
    PopScreen,
}

pub trait Screen {
    fn on_input(&mut self, input: &piston_window::Input);
    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow);
    fn on_update(&mut self, args: &piston_window::UpdateArgs);
}
