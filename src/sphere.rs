use std::sync::Arc;

use crate::{
    hittable::{self, Hittable},
    lambertian::Lambertian,
    material::Material,
    Interval, Ray,
};
use linalg::{vector::Vector, Point};

#[derive(Clone)]
pub struct Sphere {
    centre: Ray,
    radius: f64,
    material: Arc<dyn Material>,
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}

impl Sphere {
    pub fn new(
        centre: Point<f64, 3>,
        centre2: Option<Point<f64, 3>>,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        if let Some(c2) = centre2 {
            Self {
                centre: Ray::new(centre, c2 - centre, None),
                radius,
                material,
            }
        } else {
            Self {
                centre: Ray::new(centre, Vector::new([0., 0., 0.]), None),
                radius,
                material,
            }
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            centre: Ray::default(),
            radius: 0.0,
            material: Arc::new(Lambertian::default()),
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        ray_t: Interval,
        record: &mut hittable::HitRecord,
    ) -> bool {
        let current_centre = self.centre.at(ray.time());
        let oc = ray.origin() - current_centre;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return false;
        }
        let sqrt_disc = discriminant.sqrt();

        let mut root = (-half_b - sqrt_disc) / a;

        if !ray_t.surrounds(root) {
            root = (-half_b + sqrt_disc) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        record.distance = root;
        record.p = ray.at(record.distance);
        let outward_normal = (record.p - current_centre) / self.radius;
        record.set_face_normal(ray, &outward_normal);
        record.material = self.material.clone();

        true
    }
}

impl Material for Sphere {
    fn scatter(
        &self,
        ray_in: &crate::Ray,
        record: &hittable::HitRecord,
        attenuation: &mut crate::colour::Colour,
        scattered: &mut crate::Ray,
    ) -> bool {
        self.material
            .scatter(ray_in, record, attenuation, scattered)
    }
}
