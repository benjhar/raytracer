use std::sync::Arc;

use linalg::{num_traits::ops::mul_add::MulAdd, vector::Vector};

use crate::{
    engine::{hittable::HitRecord, ray::Ray},
    textures::Texture,
    thread_rng,
    util::colour::Colour,
};

use super::Material;

#[derive(Clone)]
pub struct Metal {
    albedo: Colour,
    roughness: Arc<dyn Texture>,
}

impl Metal {
    pub fn new(albedo: Colour, roughness: Arc<dyn Texture>) -> Self {
        Self { albedo, roughness }
    }
}

unsafe impl Sync for Metal {}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vector::reflect(ray_in.direction().unit(), record.normal);
        reflected = self.roughness.value(record.u, record.v, &record.p).mul_add(
            Vector::random_unit_vector_with_rng(|range| thread_rng().f64_range(range)),
            reflected.unit(),
        );

        *scattered = Ray::new(record.p, reflected, Some(ray_in.time()));
        *attenuation = self.albedo;

        scattered.direction().dot(&record.normal) > 0.0
    }
}
