use piston_window;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use gfx_graphics::GfxGraphics;
use piston_window::Transformed;

pub struct Object {
    x: f64,
    y: f64,
    sprite: Option<piston_window::Texture<Resources>>,
}

impl Object {
    pub fn new() -> Object {
        Object { x: 0., y: 0., sprite: None }
    }

    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn mov_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn render(&self, graphic: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, view: &piston_window::math::Matrix2d) {
        let red = [1.0, 0.0, 0.0, 1.0];
        let square = piston_window::rectangle::square(0.0, 0.0, 100.0);
        piston_window::rectangle(red, square, view.trans(self.x, self.y).trans(-50.0, -50.0), graphic);
    }
}
