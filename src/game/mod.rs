extern crate find_folder;

use screen::{self, Screen};

use piston_window::{self, PistonWindow, EventLoop};

type Screens = Vec<Box<screen::Screen>>;

pub struct Game {
    window: PistonWindow,
    context: screen::Context,
    screens: Screens,
}

impl Game {
    pub fn new() -> Self {
        let window: PistonWindow = piston_window::WindowSettings::new("rrthozopsi", [600, 600])
                                       .build()
                                       .unwrap();
        let mut context = screen::Context {
            assets_path: find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap(),
        };
        let screens: Screens = vec![Box::new(screen::MainMenu::new(&window, &mut context))];
        Game {
            window: window,
            screens: screens,
            context: context,
        }
    }

    pub fn run(&mut self) {
        use piston_window::Event;

        let screens = &mut self.screens;
/*
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
                    handle_input(screens, &input, &window, &mut self.context);
                    if screens.is_empty() {
                        // No screens - exit game.
                        return;
                    }
                }
                _ => {}
            }
        }
*/
        for event in self.window.clone().ups(60) {
            handle_event(screens, &event, &mut self.context);
            if screens.is_empty() {
                // No screens - exit game.
                return;
            }
        }
    }
}

fn last<'a>(vec: &'a mut Screens) -> &'a mut Box<screen::Screen> {
    vec.last_mut().unwrap()
}

fn handle_input(screens: &mut Screens,
                input: &piston_window::Input,
                window: &PistonWindow,
                context: &mut screen::Context) {
    for action in last(screens).on_input(&input, &window, context) {
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

fn handle_event(screens: &mut Screens,
                window: &PistonWindow,
                context: &mut screen::Context) {
    for action in last(screens).on_event(&window, context) {
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