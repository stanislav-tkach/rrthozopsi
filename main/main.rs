extern crate piston_window;

use piston_window as pw;

fn main() {
    let window: pw::PistonWindow = pw::WindowSettings::new("rrthozopsi", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    for e in window {
		e.draw_2d(|context, graphics| {
			pw::clear([0.0, 0.0, 0.0, 1.0], graphics);
		});
    }
}
