extern crate piston_window;

use piston_window::Transformed;

use piston_window as pw;

fn main() {
    let window: pw::PistonWindow = pw::WindowSettings::new("rrthozopsi", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    for w in window {
		w.draw_2d(|context, graphics| {
			pw::clear([0.0, 0.0, 0.0, 1.0], graphics);

			let red = [1.0, 0.0, 0.0, 1.0];
			let square = pw::rectangle::square(0.0, 0.0, 100.0);
			let center = context.transform.trans(300.0, 300.0);
			pw::rectangle(red, square, center.trans(-50.0, -50.0), graphics);
		});
    }
}
