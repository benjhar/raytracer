use linalg::Point;

use crate::colour::Colour;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point<f64, 3>) -> Colour;
}
