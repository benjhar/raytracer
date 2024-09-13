pub mod camera;
pub mod colour;
pub mod dielectric;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod lambertian;
pub mod material;
pub mod metals;
pub mod ray;
pub mod sphere;

use std::f64::consts::PI;

pub use interval::Interval;
pub use linalg::{vector::*, Point};
pub use ray::*;

#[inline]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
