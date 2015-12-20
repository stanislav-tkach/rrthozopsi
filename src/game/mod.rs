use ::screen;
use ::events;
use ::object;

use piston_window;
use piston_window::Transformed;

use gfx_device_gl;

pub struct Game {
    screens: Vec<Box<screen::Screen>>,
    events: events::Events,
    player: object::Object,
}

impl Game {
    pub fn new(sprite: piston_window::Texture<gfx_device_gl::Resources>) -> Game {
        Game { screens: Vec::new(), events: events::Events::new(), player: object::Object::new(Some(sprite)) }
    }

    pub fn run() {
    }

    pub fn on_update(&mut self, args: &piston_window::UpdateArgs) {
        if self.events.is_up()   { self.player.mov(0.0, -150.0 * args.dt); }
        if self.events.is_down() { self.player.mov(0.0, 150.0 * args.dt); }
        if self.events.is_left() { self.player.mov(-150.0 * args.dt, 0.0); }
        if self.events.is_right() { self.player.mov(150.0 * args.dt, 0.0); }
    }

    pub fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow) {
        window.draw_2d(|context, graphics| {
            piston_window::clear([0.0, 0.0, 0.0, 1.0], graphics);

            let center = context.transform.trans((args.width / 2) as f64, (args.height / 2) as f64);
            self.player.render(graphics, &center);
        });
    }

    pub fn on_input(&mut self, input: &piston_window::Input) {
        self.events.process_input(&input);
    }
}
