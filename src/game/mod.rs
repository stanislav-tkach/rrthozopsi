use screen;
use screen::Screen;

use piston_window;

type Screens = Vec<Box<screen::Screen>>;

pub struct Game {
    window: piston_window::PistonWindow,
    screens: Screens,
}

impl Game {
    pub fn new() -> Game {
        let window: piston_window::PistonWindow = piston_window::WindowSettings::new("rrthozopsi", [600, 600])
                                                      .exit_on_esc(true)
                                                      .build()
                                                      .unwrap();
        let screens: Screens = vec![Box::new(screen::MainMenuScreen::new(&window))];
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
                Some(Event::Update(ref args)) => {
                    last(screens).on_update(&args);
                }
                Some(Event::Render(ref args)) => {
                    last(screens).on_draw(&args, &window);
                }
                Some(Event::Input(ref input)) => {
                    handle_input(screens, &input, &window);
                }
                _ => {}
            }
        }
    }
}

fn last<'a>(vec: &'a mut Screens) -> &'a mut Box<screen::Screen> {
    vec.last_mut().unwrap()
}

fn handle_input(screens: &mut Screens, input: &piston_window::Input, window: &piston_window::PistonWindow) {
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
