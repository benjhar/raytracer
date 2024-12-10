pub mod bounding_volume_hierarchies;
pub mod engine;
pub mod materials;
pub mod noise;
pub mod surface;
pub mod textures;
pub mod util;

use std::f64::consts::PI;

#[inline]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
