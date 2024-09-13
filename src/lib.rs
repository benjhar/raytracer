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

use rand::{random, Rng};

// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility functions
#[inline]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
fn random_f64() -> f64 {
    random()
}

#[inline]
fn random_f64_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}
