extern crate find_folder;

use screen::*;
use piston_window;
use conrod;
use conrod::Widget;
use conrod::Frameable;
use conrod::color::Colorable;
use conrod::Positionable;

pub struct MainMenuScreen;

impl MainMenuScreen {
    pub fn new() -> MainMenuScreen {
        MainMenuScreen
    }
}

impl Screen for MainMenuScreen {
    fn on_input(&mut self, input: &piston_window::Input, window: &piston_window::PistonWindow) -> InputResults {
        use piston_window::{Input, Button, MouseButton};

        let mut result = Vec::new();

        match input {
            &Input::Press(Button::Mouse(MouseButton::Left)) => {
                result.push(InputResult::PopScreen);
                result.push(InputResult::PushScreen(Box::new(BattleScreen::new(&window))));
            }
            _ => {}
        }

        result
    }

    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow) {
        // TODO: Move Ui to some context?
        // piston_window::Glyphs
        // type Ui = conrod::Ui<Glyphs>;
        let mut ui = {
            let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
            let font_path = assets.join("NotoSans-Regular.ttf");
            let theme = conrod::Theme::default();
            let glyph_cache = piston_window::Glyphs::new(&font_path, window.factory.borrow().clone());
            conrod::Ui::new(glyph_cache.unwrap(), theme)
        };

        ui.handle_event(window);

        ui.set_widgets(|ui| {
            conrod::Canvas::new()
                .frame(200.)
                .pad(30.0)
                .color(conrod::color::rgb(0.2, 0.35, 0.45))
                .scroll_kids()
                .set(CANVAS, ui);

            conrod::Text::new("Widget Demonstration")
                .top_left_with_margins_on(CANVAS, 0.0, 350.0)
                .font_size(32)
                .color(conrod::color::rgb(0.2, 0.35, 0.45).plain_contrast())
                .set(TITLE, ui);
        });

        window.draw_2d(|context, graphics| ui.draw(context, graphics));
    }

    fn on_update(&mut self, args: &piston_window::UpdateArgs) {}
}

widget_ids! {
    CANVAS,
    TITLE,
}
