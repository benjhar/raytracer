use linalg::Point;

use crate::util::colour::Colour;

use super::Texture;

pub struct Solid {
    albedo: Colour,
}

impl Solid {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            albedo: Colour::new([r, g, b]),
        }
    }

    pub fn from_colour(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Texture for Solid {
    fn value(&self, _: f64, _: f64, _: &Point<f64, 3>) -> Colour {
        self.albedo
    }
}
