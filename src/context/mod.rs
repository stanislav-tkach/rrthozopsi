use resource_manager::Manager;

#[derive(PartialEq, Clone, Debug)]
pub enum GameState {
    NotStarted,
    InProgress,
}

pub struct Context {
    pub resource_manager: Manager,
    pub game_state: GameState,
}
