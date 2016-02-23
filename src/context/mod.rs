use std::path::PathBuf;

pub enum GameState {
    NotStarted,
    InProgress,
}

pub struct Context {
    pub assets_path: PathBuf,
    pub game_state: GameState,
}
