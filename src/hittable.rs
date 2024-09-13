use crate::{material::Materials, ray::Ray, Interval};
use linalg::{vector::Vector, Point};

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub p: Point<f64, 3>,
    pub normal: Vector<f64, 3>,
    pub material: Materials,
    pub distance: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// Sets the hit normal vector.
    /// NOTE: the parameter `outward_normal` is assumed to have unit length.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector<f64, 3>) {
        self.front_face = ray.direction().dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;
}
