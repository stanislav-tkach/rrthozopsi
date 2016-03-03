use std::path::Path;
use piston_window::{PistonWindow, G2d, Glyphs};
use conrod::{self, Theme, Graphics};

type Backend = (<G2d<'static> as Graphics>::Texture, Glyphs);
pub type Ui = conrod::Ui<Backend>;

pub fn create_ui(window: &PistonWindow, assets_path: &Path) -> Ui {
    let font_path = assets_path.join("NotoSans-Regular.ttf");
    let theme = Theme::default();
    let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
    Ui::new(glyph_cache.unwrap(), theme)
}
