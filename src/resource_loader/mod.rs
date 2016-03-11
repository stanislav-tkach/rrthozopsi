extern crate find_folder;

use std::path::PathBuf;

pub struct Loader {
    assets_path: PathBuf,
}

impl Loader {
    pub fn new() -> Self {
        Loader { assets_path: find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap() }
    }

    pub fn get_asset_path(&self, name: &str) -> PathBuf {
        self.assets_path.join(name)
    }
}
