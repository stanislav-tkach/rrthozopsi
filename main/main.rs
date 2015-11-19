extern crate piston_window;

use piston_window::Transformed;

use piston_window as pw;

struct Game {
    rotation: f64,
	x: f64,
	y: f64,
	up: bool,
	down: bool,
	left: bool,
	right: bool,
}

impl Game {
    fn new() -> Game {
        Game { rotation: 0.0, x: 0., y: 0., up: false, down: false, left: false, right: false }
    }

    fn on_update(&mut self, args: &pw::UpdateArgs) {
        self.rotation += 3.0 * args.dt;

		if self.up {
			self.y += -50.0 * args.dt;
		}
		if self.down {
			self.y += 50.0 * args.dt;
		}
		if self.left {
			self.x += -50.0 * args.dt;
		}
		if self.right {
			self.x += 50.0 * args.dt;
		}
    }

    fn on_draw(&mut self, args: &pw::RenderArgs, window: &pw::PistonWindow) {
        window.draw_2d(|context, graphics| {
            pw::clear([0.0, 0.0, 0.0, 1.0], graphics);

            let red = [1.0, 0.0, 0.0, 1.0];
            let square = pw::rectangle::square(0.0, 0.0, 100.0);
            let center = context.transform.trans((args.width / 2) as f64, (args.height / 2) as f64);
            pw::rectangle(red, square, center.trans(self.x, self.y).rot_rad(self.rotation).trans(-50.0, -50.0), graphics);
        });
    }

	fn on_input(&mut self, input: &pw::Input) {
		use piston_window::{Button, Key};

		match input {
			&pw::Input::Press(button) => {
				match button {
					Button::Keyboard(Key::Up) => { self.up = true; }
					Button::Keyboard(Key::Down) => { self.down = true; }
					Button::Keyboard(Key::Left) => { self.left = true; }
					Button::Keyboard(Key::Right) => { self.right = true; }
					_ => {}
				}
			}
			&pw::Input::Release(button) => {
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

fn main() {
	use piston_window::Event;

    let window: pw::PistonWindow = pw::WindowSettings::new("rrthozopsi", [600, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

	let mut game = Game::new();

    for w in window {
		match w.event {
			Some(Event::Update(args)) => { game.on_update(&args); }
			Some(Event::Render(args)) => { game.on_draw(&args, &w); }
			Some(Event::Input(input)) => { game.on_input(&input); }
			_ => {}
		}
    }
}
