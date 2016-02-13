use gfx_device_gl;
use gfx_graphics::GfxGraphics;

use piston_window;
use piston_window::Transformed;
use piston_window::ImageSize;

#[derive(Debug)]
pub struct Object {
    x: f64,
    y: f64,
    // TODO: Remove Option.
    sprite: Option<piston_window::Texture<gfx_device_gl::Resources>>,
}

impl Object {
    pub fn new(sprite: Option<piston_window::Texture<gfx_device_gl::Resources>>) -> Self {
        Object {
            x: 0.,
            y: 0.,
            sprite: sprite,
        }
    }

    pub fn mov(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn mov_to(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    pub fn render(&self,
                  graphic: &mut GfxGraphics<gfx_device_gl::Resources,
                                            gfx_device_gl::CommandBuffer<gfx_device_gl::Resources>,
                                            gfx_device_gl::Output>,
                  view: &piston_window::math::Matrix2d) {
        if let Some(ref sprite) = self.sprite {
            let (x, y) = sprite.get_size();
            let (x, y) = (x / 2, y / 2);
            piston_window::image(sprite,
                                 view.trans(self.x, self.y).trans(-(x as f64), -(y as f64)),
                                 graphic);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_new() {
        let object = Object::new(None);
        assert_eq!(object.x, 0.0);
        assert_eq!(object.y, 0.0);
    }

    #[test]
    fn object_mov() {
        let mut object = Object::new(None);
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
        let mut object = Object::new(None);
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
}
