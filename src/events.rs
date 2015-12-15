
pub struct Events {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Events {
    pub fn is_up() -> bool {
        self.up
    }

    pub fn is_down() -> bool {
        self.down
    }

    pub fn is_left() -> bool {
        self.left
    }

    pub fn is_right() -> bool {
        self.right
    }
}

