pub mod camera;
pub mod colour;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod lambertian;
pub mod material;
pub mod metals;
pub mod ray;
pub mod sphere;

pub use interval::Interval;
pub use linalg::{vector::*, Point};
pub use ray::*;

use rand::random;

// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility functions
#[inline]
fn random_f64() -> f64 {
    random()
}
