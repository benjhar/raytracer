use std::sync::Arc;

use crate::util::colour::Colour;

use super::Texture;

pub struct Invert {
    texture: Arc<dyn Texture>,
}

unsafe impl Send for Invert {}
unsafe impl Sync for Invert {}

impl Invert {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Texture for Invert {
    fn value(&self, u: f64, v: f64, p: &linalg::Point<f64, 3>) -> Colour {
        -1.0 * self.texture.value(u, v, p) + Colour::one()
    }
}
