
pub struct Events {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Events {
    fn is_up() -> bool {
        self.up
    }

    fn is_down() -> bool {
        self.down
    }

    fn is_left() -> bool {
        self.left
    }

    fn is_right() -> bool {
        self.right
    }
}

