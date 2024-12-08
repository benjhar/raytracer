use linalg::Point;

use crate::{colour::Colour, texture::Texture};

pub struct SolidColour {
    albedo: Colour,
}

impl SolidColour {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColour {
    fn value(&self, _: f64, _: f64, _: &Point<f64, 3>) -> Colour {
        self.albedo
    }
}
