use screen::*;
use piston_window;

pub struct BattleScreen {
    // TODO: FIXME.
    _fixme: i32,
//    player: object::Object,
}

impl BattleScreen {
    pub fn new() -> BattleScreen {
        BattleScreen{ _fixme: 42 }
    }
}

impl Screen for BattleScreen {
    fn on_input(&mut self, input: &piston_window::Input) -> InputResults {
        use piston_window::{Input, Button, Key};

        let result = Vec::new();

        match input {
            &Input::Press(button) => {
                match button {
//                    Button::Keyboard(Key::Up) => { self.up = true; }
//                    Button::Keyboard(Key::Down) => { self.down = true; }
//                    Button::Keyboard(Key::Left) => { self.left = true; }
//                    Button::Keyboard(Key::Right) => { self.right = true; }
                    _ => {}
                }
            }
            &Input::Release(button) => {
                match button {
//                    Button::Keyboard(Key::Up) => { self.up = false; }
//                    Button::Keyboard(Key::Down) => { self.down = false; }
//                    Button::Keyboard(Key::Left) => { self.left = false; }
//                    Button::Keyboard(Key::Right) => { self.right = false; }
                    _ => {}
                }
            }
            _ => {}
        }

        result
    }

    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow) {
    }

    fn on_update(&mut self, args: &piston_window::UpdateArgs) {
    }
}
