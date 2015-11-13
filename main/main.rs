extern crate piston_window;

fn main() {
    let window: piston_window::PistonWindow = piston_window::WindowSettings::new("piston-tutorial", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    for _e in window {
        break;
    }
}
