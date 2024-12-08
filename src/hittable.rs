use std::sync::Arc;

use crate::{
    aabb::AABB, colour::Colour, lambertian::Lambertian, material::Material, ray::Ray, Interval,
};
use linalg::{vector::Vector, Point};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point<f64, 3>,
    pub normal: Vector<f64, 3>,
    pub material: Arc<dyn Material>,
    pub distance: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point::default(),
            normal: Vector::default(),
            material: Arc::new(Lambertian::from_colour(Colour::new([1.; 3]))),
            distance: 0.0,
            u: 0.,
            v: 0.,
            front_face: true,
        }
    }
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

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB;
}

impl Hittable for &dyn Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        (*self).hit(ray, ray_t, record)
    }

    fn bounding_box(&self) -> AABB {
        (*self).bounding_box()
    }
}
