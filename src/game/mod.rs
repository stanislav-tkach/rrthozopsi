extern crate find_folder;

use ::screen;
use ::screen::Screen;

use piston_window;
use piston_window::Transformed;

use gfx_device_gl;

pub struct Game {
    window: piston_window::PistonWindow,
    screens: Vec<Box<screen::Screen>>,
}

impl Game {
    pub fn new() -> Game {
        let window: piston_window::PistonWindow = piston_window::WindowSettings::new("rrthozopsi", [600, 600]).exit_on_esc(true).build().unwrap();
        let screens: Vec<Box<screen::Screen>> = vec![Box::new(screen::MainMenuScreen::new())];
        Game { 
            window: window,
            screens: screens, 
        }
    }

    pub fn run(&mut self) {
        use piston_window::Event;

        let mut screen = self.screens.last_mut().unwrap();

        // TODO: Remove clone?
        for window in &mut self.window {
            match window.event {
                Some(Event::Update(args)) => { screen.on_update(&args); }
                Some(Event::Render(args)) => { screen.on_draw(&args, &window); }
                Some(Event::Input(input)) => { screen.on_input(&input); }
                _ => {}
            }
        }
    }
}
