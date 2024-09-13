use linalg::vector::Vector;

use crate::{colour::Colour, hittable::HitRecord, Ray};

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + Vector::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        *scattered = Ray::new(record.p, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

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

        *scattered = Ray::new(record.p, reflected);
        *attenuation = self.albedo;

        scattered.direction().dot(&record.normal) > 0.0
    }
}
