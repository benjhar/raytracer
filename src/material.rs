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

#[derive(Clone, Copy, Debug)]
pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Materials {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Materials::Lambertian(l) => l.scatter(ray_in, record, attenuation, scattered),
            Materials::Metal(m) => m.scatter(ray_in, record, attenuation, scattered),
        }
    }
}

impl Default for Materials {
    fn default() -> Self {
        Materials::Lambertian(Lambertian::default())
    }
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
        let mut scatter_direction = record.normal.hadamard(Vector::random_unit_vector());

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
}

impl Metal {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
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
        let reflected = Vector::reflect(ray_in.direction().unit(), record.normal);
        *scattered = Ray::new(record.p, reflected);
        *attenuation = self.albedo;

        true
    }
}
