use std::path::Path;
use piston_window::{self, PistonWindow};
use conrod;

pub fn create_ui(window: &PistonWindow, assets_path: &Path) -> conrod::Ui<(<piston_window::G2d<'static> as conrod::Graphics>::Texture, piston_window::Glyphs)> {
    let font_path = assets_path.join("NotoSans-Regular.ttf");
    let theme = conrod::Theme::default();
    let glyph_cache = piston_window::Glyphs::new(&font_path, window.factory.borrow().clone());
    conrod::Ui::new(glyph_cache.unwrap(), theme)
}
