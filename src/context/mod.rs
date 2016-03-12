use resource_loader::Loader;

#[derive(PartialEq)]
pub enum GameState {
    NotStarted,
    InProgress,
}

pub struct Context {
    pub loader: Loader,
    pub game_state: GameState,
}
