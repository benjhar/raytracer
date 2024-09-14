use linalg::vector::Vector;

use crate::{colour::Colour, hittable::HitRecord, material::Material, Ray};

#[derive(Clone, Copy, Default, Debug)]
pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vector::reflect(ray_in.direction().unit(), record.normal);
        reflected = reflected.unit() + (self.fuzz * Vector::random_unit_vector());

        *scattered = Ray::new(record.p, reflected, Some(ray_in.time()));
        *attenuation = self.albedo;

        scattered.direction().dot(&record.normal) > 0.0
    }
}
