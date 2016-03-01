use screen::*;
use context::{Context, GameState};
use ui;
use piston_window::{self, PistonWindow, Glyphs};
use conrod::{self, Canvas, Button, Widget, Positionable, Sizeable, Labelable};
use conrod::color::{self, Colorable};

pub struct MainMenu {
//    ui: conrod::Ui<piston_window::Glyphs>,
    ui: conrod::Ui<(<piston_window::G2d<'static> as conrod::Graphics>::Texture, piston_window::Glyphs)>,
    state: Option<State>,
}

impl MainMenu {
    pub fn new(window: &PistonWindow, context: &mut Context) -> Self {
        MainMenu {
            ui: ui::create_ui(&window, &context.assets_path),
            state: None,
        }
    }
}

enum State {
    NewGame,
    Options,
    Exit,
}

impl Screen for MainMenu {
    fn on_input(&mut self, _: &piston_window::Input, window: &PistonWindow, context: &mut Context) -> InputResults {
        self.ui.handle_event(window);
        let mut result = Vec::new();

        match self.state.take() {
            Some(State::NewGame) => {
                result.push(InputResult::PopScreen);
                result.push(InputResult::PushScreen(Box::new(Battle::new(&window, context))));
                context.game_state = GameState::InProgress;
            }
            Some(State::Options) => {
                result.push(InputResult::PushScreen(Box::new(Options::new(&window, context))));
            }
            Some(State::Exit) => {
                result.push(InputResult::Exit);
            }
            None => {}
        }

        result
    }

    fn on_draw(&mut self, _: &piston_window::RenderArgs, window: &PistonWindow) {
        self.ui.handle_event(window);
        window.draw_2d(|context, graphics| self.ui.draw_if_changed(context, graphics));
    }

    fn on_update(&mut self, _: &piston_window::UpdateArgs) {
        let button_color = color::rgb(0.4, 0.75, 0.6);

        let state = &mut self.state;

        self.ui.set_widgets(|ui| {
            Canvas::new()
                .pad(30.)
                .color(color::rgb(0.2, 0.35, 0.45))
                .scroll_kids()
                .set(CANVAS, ui);

            Button::new()
                .w_h(200.0, 50.0)
                .mid_left_of(CANVAS)
                .color(button_color)
                .label("New game")
                .react(|| *state = Some(State::NewGame))
                .set(NEW_GAME_BUTTON, ui);

            Button::new()
                .w_h(200.0, 50.0)
                .mid_left_of(CANVAS)
                .down_from(NEW_GAME_BUTTON, 45.0)
                .color(button_color)
                .label("Options")
                .react(|| *state = Some(State::Options))
                .set(OPTIONS_BUTTON, ui);

            Button::new()
                .w_h(200.0, 50.0)
                .mid_left_of(CANVAS)
                .down_from(OPTIONS_BUTTON, 45.0)
                .color(button_color)
                .label("Exit")
                .react(|| *state = Some(State::Exit))
                .set(EXIT_BUTTON, ui);
        });
    }
}

widget_ids! {
    CANVAS,
    NEW_GAME_BUTTON,
    OPTIONS_BUTTON,
    EXIT_BUTTON,
}
