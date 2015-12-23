mod screen;
mod main_menu_screen;
mod battle_screen;

pub use self::screen::*;
pub use self::main_menu_screen::*;
pub use self::battle_screen::*;












/*
use piston_window;

mod main_menu_screen;
mod battle_screen;

pub trait Screen {
    fn on_input(&mut self, input: &piston_window::Input);
    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow);
    fn on_update(&mut self, args: &piston_window::UpdateArgs);
}

pub use self::main_menu_screen::*;
pub use self::battle_screen::*;
*/