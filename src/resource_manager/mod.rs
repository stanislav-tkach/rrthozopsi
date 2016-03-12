extern crate find_folder;

use std::path::PathBuf;

pub struct Manager {
    assets_path: PathBuf,
}

impl Manager {
    pub fn new() -> Self {
        Manager { assets_path: find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap() }
    }

    pub fn get_asset_path(&self, name: &str) -> PathBuf {
        self.assets_path.join(name)
    }

    pub fn get_font(&self) -> PathBuf {
        self.assets_path.join("NotoSans-Regular.ttf")
    }
}
