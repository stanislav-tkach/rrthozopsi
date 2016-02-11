use screen::*;
use piston_window;
use conrod::{self, Widget, Frameable, Positionable, Sizeable, Labelable};
use conrod::color::Colorable;

pub struct MainMenuScreen {
    ui: conrod::Ui<piston_window::Glyphs>,
    state: Option<State>,
}

impl MainMenuScreen {
    pub fn new(window: &piston_window::PistonWindow, context: &mut Context) -> MainMenuScreen {
        MainMenuScreen {
            ui: create_ui(&window, &context.assets_path),
            state: None,
        }
    }
}

enum State {
    NewGame,
    Options,
    Exit,
}

impl Screen for MainMenuScreen {
    fn on_input(&mut self,
                _: &piston_window::Input,
                window: &piston_window::PistonWindow,
                context: &mut Context)
                -> InputResults {
        self.ui.handle_event(window);
        let mut result = Vec::new();

        match self.state {
            Some(State::NewGame) => {
                result.push(InputResult::PopScreen);
                result.push(InputResult::PushScreen(Box::new(BattleScreen::new(&window, context))));
            }
            Some(State::Options) => {}
            Some(State::Exit) => {
                result.push(InputResult::PopScreen);
            }
            None => {}
        }

        result
    }

    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow) {
        self.ui.handle_event(window);
        window.draw_2d(|context, graphics| self.ui.draw_if_changed(context, graphics));
    }

    fn on_update(&mut self, args: &piston_window::UpdateArgs) {
        let state = &mut self.state;

        self.ui.set_widgets(|ui| {
            conrod::Canvas::new()
                .pad(30.)
                .color(conrod::color::rgb(0.2, 0.35, 0.45))
                .scroll_kids()
                .set(CANVAS, ui);

            conrod::Button::new()
                .w_h(200.0, 50.0)
                .mid_left_of(CANVAS)
                .rgb(0.4, 0.75, 0.6)
                .label("New game")
                .react(|| *state = Some(State::NewGame))
                .set(NEW_GAME_BUTTON, ui);

            conrod::Button::new()
                .w_h(200.0, 50.0)
                .mid_left_of(CANVAS)
                .down_from(NEW_GAME_BUTTON, 45.0)
                .rgb(0.4, 0.75, 0.6)
                .label("Options")
                .react(|| *state = Some(State::Options))
                .set(OPTIONS_BUTTON, ui);

            conrod::Button::new()
                .w_h(200.0, 50.0)
                .mid_left_of(CANVAS)
                .down_from(OPTIONS_BUTTON, 45.0)
                .rgb(0.4, 0.75, 0.6)
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

fn create_ui(window: &piston_window::PistonWindow,
             assets_path: &::std::path::Path)
             -> conrod::Ui<piston_window::Glyphs> {
    let font_path = assets_path.join("NotoSans-Regular.ttf");
    let theme = conrod::Theme::default();
    let glyph_cache = piston_window::Glyphs::new(&font_path, window.factory.borrow().clone());
    conrod::Ui::new(glyph_cache.unwrap(), theme)
}
