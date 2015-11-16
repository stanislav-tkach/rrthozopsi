extern crate piston_window;

use piston_window::Transformed;

use piston_window as pw;

struct Game {
    rotation: f64
}

impl Game {
    fn new() -> Game {
        Game { rotation : 0.0 }
    }

    fn on_update(&mut self, args: pw::UpdateArgs) {
        self.rotation += 3.0 * args.dt;
    }

    fn on_draw(&mut self, args: pw::RenderArgs, window: pw::PistonWindow) {
        window.draw_2d(|context, graphics| {
            pw::clear([0.0, 0.0, 0.0, 1.0], graphics);

            let red = [1.0, 0.0, 0.0, 1.0];
            let square = pw::rectangle::square(0.0, 0.0, 100.0);
            let center = context.transform.trans((args.width / 2) as f64, (args.height / 2) as f64);
            pw::rectangle(red, square, center.rot_rad(self.rotation).trans(-50.0, -50.0), graphics);
        });
    }
}

fn main() {
    let window: pw::PistonWindow = pw::WindowSettings::new("rrthozopsi", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

	let mut game = Game::new();

    for w in window {
		match w.event {
			Some(pw::Event::Update(args)) => { game.on_update(args); }
			Some(pw::Event::Render(args)) => { game.on_draw(args, w); }
			_ => {}
		}
    }
}
