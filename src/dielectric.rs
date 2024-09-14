use linalg::vector::Vector;
use rand::random;

use crate::{colour::Colour, material::Material, Ray};

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let mut r0 = (1.0 - refractive_index) / (1. + refractive_index);
        r0 = r0 * r0;

        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &crate::Ray,
        record: &crate::hittable::HitRecord,
        attenuation: &mut crate::colour::Colour,
        scattered: &mut crate::Ray,
    ) -> bool {
        *attenuation = Colour::new([1.0, 1.0, 1.0]);
        let ri = if record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray_in.direction().unit();

        let cos_theta = (-unit_direction).dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random::<f64>() {
            Vector::reflect(unit_direction, record.normal)
        } else {
            Vector::refract(&unit_direction, &record.normal, ri)
        };

        *scattered = Ray::new(record.p, direction, Some(ray_in.time()));

        true
    }
}
