extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};

fn main() {
    let window: PistonWindow = WindowSettings::new("rrthozopsi", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    for _e in window {
    }
}
