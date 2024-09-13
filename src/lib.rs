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
