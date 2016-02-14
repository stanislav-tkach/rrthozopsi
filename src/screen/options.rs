use screen::*;
use piston_window::{self, PistonWindow};
use conrod::{self, Widget, Positionable, Sizeable, Labelable};
use conrod::color::Colorable;

pub struct Options {
    ui: conrod::Ui<piston_window::Glyphs>,
    back: bool,
}