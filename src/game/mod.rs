use ::screen;
use ::screen::Screen;

use piston_window;

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

        let screens = &mut self.screens;

        // TODO: Remove clone?
        for window in &mut self.window {
            match window.event {
                Some(Event::Update(ref args)) => { last(screens).on_update(&args); }
                Some(Event::Render(ref args)) => { last(screens).on_draw(&args, &window); }
                Some(Event::Input(ref input)) => {
                    for action in last(screens).on_input(&input, &window) {
                        match action {
                            screen::InputResult::PushScreen(new_screen) => {
                                screens.push(new_screen);
                            }
            		        screen::InputResult::PopScreen => {
            			        screens.pop();
            		        }
                        }
                    }
            	}
                _ => {}
            }
        }
    }
}

fn last<'a>(vec: &'a mut Vec<Box<screen::Screen>>) -> &'a mut Box<screen::Screen> {
    vec.last_mut().unwrap()
}