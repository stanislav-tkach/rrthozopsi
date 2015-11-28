use piston_window;
use gfx_device_gl::{Resources, Output, CommandBuffer};
use gfx_graphics::GfxGraphics;

use piston_window::Transformed;
use piston_window::ImageSize;

#[derive(Debug)]
pub struct Object {
    x: f64,
    y: f64,
    sprite: piston_window::Texture<Resources>,
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

    pub fn set_sprite(&mut self, sprite: piston_window::Texture<Resources>) {
        self.sprite = Some(sprite);
    }

    pub fn render(&self, graphic: &mut GfxGraphics<Resources, CommandBuffer<Resources>, Output>, view: &piston_window::math::Matrix2d) {
        let (x, y) = sprite.get_size();
        let (x, y) = (x / 2, y / 2);
        piston_window::image(sprite, view.trans(self.x, self.y).trans(-(x as f64), -(y as f64)), graphic);
    }
}

#[test]
fn object_new() {
    let object = Object::new();
    assert_eq!(object.x, 0.0);
    assert_eq!(object.y, 0.0);
}

#[test]
fn object_mov() {
    let mut object = Object::new();
    assert_eq!(object.x, 0.);
    assert_eq!(object.y, 0.);

    object.mov(0., 0.);
    assert_eq!(object.x, 0.);
    assert_eq!(object.y, 0.);

    object.mov(1., 1.);
    assert_eq!(object.x, 1.);
    assert_eq!(object.y, 1.);

    object.mov(-3., -3.);
    assert_eq!(object.x, -2.);
    assert_eq!(object.y, -2.);
}

#[test]
fn object_mov_to() {
    let mut object = Object::new();
    assert_eq!(object.x, 0.);
    assert_eq!(object.y, 0.);

    object.mov_to(0., 0.);
    assert_eq!(object.x, 0.);
    assert_eq!(object.y, 0.);

    object.mov_to(10., 10.);
    assert_eq!(object.x, 10.);
    assert_eq!(object.y, 10.);

    object.mov_to(-30., -30.);
    assert_eq!(object.x, -30.);
    assert_eq!(object.y, -30.);
}
