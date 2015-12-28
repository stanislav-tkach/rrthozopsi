use screen::*;
use piston_window;

pub struct BattleScreen {
    // TODO: FIXME.
    _fixme: i32,
//    player: object::Object,
}

impl BattleScreen {
    fn new() -> BattleScreen {
        BattleScreen{ _fixme: 42 }
    }
}

impl Screen for BattleScreen {
    fn on_input(&mut self, input: &piston_window::Input) -> InputResults {
        let result = Vec::new();
        result
    }

    fn on_draw(&mut self, args: &piston_window::RenderArgs, window: &piston_window::PistonWindow) {
    }

    fn on_update(&mut self, args: &piston_window::UpdateArgs) {
    }
}
