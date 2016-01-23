extern crate find_folder;

use screen::*;
use piston_window;
use conrod;

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
        window.draw_2d(|context, graphics| {
            piston_window::clear([0., 0., 1., 1.0], graphics);

////////////////////////////////////////////////////////////////////////////////////
        // TODO: Move Ui to some context?
        // piston_window::Glyphs
        // type Ui = conrod::Ui<Glyphs>;
        let mut ui = {
            let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
            let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
            let theme = conrod::Theme::default();
            let glyph_cache = piston_window::Glyphs::new(&font_path, window.factory.borrow().clone());
            conrod::Ui::new(glyph_cache.unwrap(), theme)
        };

        let canvas = 0;

        conrod::Canvas::new()
            .frame(200)
            .pad(30.0)
            .color(conrod::color::rgb(0.2, 0.35, 0.45))
            .scroll_kids()
            .set(canvas, ui);

        let title = 1;

        conrod::Text::new("Widget Demonstration")
            .top_left_with_margins_on(canvas, 0.0, 350.0)
            .font_size(32)
            .color(conrod::color::rgb(0.2, 0.35, 0.45).plain_contrast())
            .set(title, ui);
////////////////////////////////////////////////////////////////////////////////////
        });
    }

    fn on_update(&mut self, args: &piston_window::UpdateArgs) {}
}
