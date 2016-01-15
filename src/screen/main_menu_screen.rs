use screen::*;
use piston_window;

pub struct MainMenuScreen;

impl MainMenuScreen {
    pub fn new() -> MainMenuScreen {
        MainMenuScreen
    }
}

impl Screen for MainMenuScreen {
    fn on_input(&mut self, input: &piston_window::Input, window: &piston_window::PistonWindow) -> InputResults {
        use piston_window::{Input, Button, MouseButton};

        let mut result = Vec::new();

        match input {
            &Input::Press(Button::Mouse(MouseButton::Left)) => {
                result.push(InputResult::PopScreen);
                result.push(InputResult::PushScreen(Box::new(BattleScreen::new(&window))));
            }
            _ => {}
        }

        result
    }

    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow) {
        window.draw_2d(|context, graphics| {
            piston_window::clear([0., 0., 1., 1.0], graphics);
        });
    }

    fn on_update(&mut self, args: &piston_window::UpdateArgs) {}
}
