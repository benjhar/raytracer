use linalg::Point;

use crate::util::colour::Colour;

use super::Texture;

pub struct Solid {
    albedo: Colour,
}

impl Solid {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Texture for Solid {
    fn value(&self, _: f64, _: f64, _: &Point<f64, 3>) -> Colour {
        self.albedo
    }
}
