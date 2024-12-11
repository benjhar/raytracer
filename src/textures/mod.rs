mod checker;
mod image;
mod invert;
mod noise;
mod solid;

pub use checker::Checker;
pub use image::Image;
pub use invert::Invert;
pub use noise::Fractal;
pub use solid::Solid;

use linalg::Point;

use crate::util::colour::Colour;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point<f64, 3>) -> Colour;
}
