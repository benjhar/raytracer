pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod checker_texture;
pub mod colour;
pub mod dielectric;
pub mod hittable;
pub mod hittable_list;
pub mod image_texture;
pub mod interval;
pub mod lambertian;
pub mod material;
pub mod metals;
pub mod noise_texture;
pub mod perlin;
pub mod ray;
pub mod rtw_image;
pub mod solid_colour;
pub mod sphere;
pub mod texture;

use std::f64::consts::PI;

pub use interval::Interval;
pub use linalg::{vector::*, Point};
pub use ray::*;

#[inline]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
