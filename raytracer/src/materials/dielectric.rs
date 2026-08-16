use std::sync::Arc;

use linalg::{num_traits::ops::mul_add::MulAdd, vector::Vector};

use crate::{
    engine::{hittable::HitRecord, ray::Ray},
    textures::Texture,
    thread_rng,
    util::colour::Colour,
};

use super::Material;

pub struct Dielectric {
    colour: Colour,
    refractive_index: f64,
    roughness: Arc<dyn Texture>,
}

unsafe impl Sync for Dielectric {}

impl Dielectric {
    pub fn new(colour: Colour, refractive_index: f64, roughness: Arc<dyn Texture>) -> Self {
        Self {
            colour,
            refractive_index,
            roughness,
        }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        let mut r0 = (1.0 - refractive_index) / (1. + refractive_index);
        r0 = r0 * r0;

        (1. - r0).mul_add((1. - cosine).powi(5), r0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = self.colour;
        let ri = if record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray_in.direction().unit();

        #[expect(clippy::arithmetic_side_effects, reason = "Floats")]
        let cos_theta = (-unit_direction).dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let mut rng = thread_rng();
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > rng.f64_inclusive()
        {
            self.roughness.value(record.u, record.v, &record.p).mul_add(
                Vector::random_unit_vector_with_rng(|range| rng.f64_range(range)),
                Vector::reflect(unit_direction, record.normal).unit(),
            )
        } else {
            Vector::refract(&unit_direction, &record.normal, ri)
        };

        *scattered = Ray::new(record.p, direction, Some(ray_in.time()));

        true
    }
}
