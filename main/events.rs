pub struct Events {
    pump: ::sdl2::EventPump,

    pub quit: bool,
    pub key_escape: bool,
}

impl Events {
    pub fn new(pump: ::sdl2::EventPump) -> Events {
        Events {
            pump: pump,

            quit: false,
            key_escape: false,
        }
    }

    pub fn pump(&mut self) {
        use ::sdl2::event::Event::{Quit, KeyDown, KeyUp};
        use ::sdl2::keyboard::Keycode::Escape;

        for event in self.pump.poll_iter() {
            match event {
                Quit { .. } => self.quit = true,

                KeyDown { keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = true,
                    _ => {}
                },

                KeyUp { keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = false,
                    _ => {}
                },

                _ => {}
            }
        }
    }
}
