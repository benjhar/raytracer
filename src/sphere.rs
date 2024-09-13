use std::rc::Rc;

use crate::{
    hittable::{self, Hittable},
    lambertian::Lambertian,
    material::Material,
    Interval,
};
use linalg::Point;

#[derive(Clone)]
pub struct Sphere {
    centre: Point<f64, 3>,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(centre: Point<f64, 3>, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            centre,
            radius,
            material,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            centre: Point::default(),
            radius: 0.0,
            material: Rc::new(Lambertian::default()),
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
        let oc = ray.origin() - self.centre;
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
        let outward_normal = (record.p - self.centre) / self.radius;
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
