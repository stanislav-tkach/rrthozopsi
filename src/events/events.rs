
pub struct Events {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Events {
    pub fn is_up() -> bool {
        self.up
    }

    pub fn is_down() -> bool {
        self.down
    }

    pub fn is_left() -> bool {
        self.left
    }

    pub fn is_right() -> bool {
        self.right
    }

    pub fn process_input(&mut self, input: &piston_window::Input) {
        use piston_window::{Input, Button, Key};

        match input {
            &Input::Press(button) => {
                match button {
                    Button::Keyboard(Key::Up) => { self.up = true; }
                    Button::Keyboard(Key::Down) => { self.down = true; }
                    Button::Keyboard(Key::Left) => { self.left = true; }
                    Button::Keyboard(Key::Right) => { self.right = true; }
                    _ => {}
                }
            }
            &Input::Release(button) => {
                match button {
                    Button::Keyboard(Key::Up) => { self.up = false; }
                    Button::Keyboard(Key::Down) => { self.down = false; }
                    Button::Keyboard(Key::Left) => { self.left = false; }
                    Button::Keyboard(Key::Right) => { self.right = false; }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
