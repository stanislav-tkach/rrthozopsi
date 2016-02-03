use screen::*;
use piston_window;
use conrod::{self, Widget, Frameable, Positionable, Sizeable, Labelable};
use conrod::color::Colorable;
//use gif::traits::SetParameter;

pub struct MainMenuScreen {
    ui: conrod::Ui<piston_window::Glyphs>,
}

impl MainMenuScreen {
    pub fn new(window: &piston_window::PistonWindow, context: &mut Context) -> MainMenuScreen {
        MainMenuScreen { ui: create_ui(&window, &context.assets_path) }
    }
}

impl Screen for MainMenuScreen {
    fn on_input(&mut self,
                input: &piston_window::Input,
                window: &piston_window::PistonWindow,
                context: &mut Context)
                -> InputResults {
        use piston_window::{Input, Button, MouseButton};

        let mut result = Vec::new();

        match input {
            &Input::Press(Button::Mouse(MouseButton::Left)) => {
                result.push(InputResult::PopScreen);
                result.push(InputResult::PushScreen(Box::new(BattleScreen::new(&window, context))));
            }
            _ => {}
        }

        result
    }

    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow) {
        self.ui.handle_event(window);
        window.draw_2d(|context, graphics| self.ui.draw_if_changed(context, graphics));
    }

    fn on_update(&mut self, args: &piston_window::UpdateArgs) {
        self.ui.set_widgets(|ui| {
            conrod::Canvas::new()
                .frame(200.)
                .pad(30.0)
                .color(conrod::color::rgb(0.2, 0.35, 0.45))
                .scroll_kids()
                .set(CANVAS, ui);

            conrod::Text::new("Main menu")
                .top_left_with_margins_on(CANVAS, 0.0, 350.0)
                .font_size(32)
                .color(conrod::color::rgb(0.2, 0.35, 0.45).plain_contrast())
                .set(TITLE, ui);

            conrod::Button::new()
//                .w_h(200.0, 50.0)
//                .mid_left_of(CANVAS)
//                .down_from(TITLE, 45.0)
                .rgb(0.4, 0.75, 0.6)
                .frame(100.)
                .label("New game")
//                .react(conrod::color::rgb(0.2, 0.35, 0.45))
                .react(|| ())
                .set(NEW_GAME_BUTTON, ui);
        });
    }
}

widget_ids! {
    CANVAS,
    TITLE,
    NEW_GAME_BUTTON,
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
