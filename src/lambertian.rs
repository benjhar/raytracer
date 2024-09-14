use linalg::vector::Vector;

use crate::{colour::Colour, hittable::HitRecord, material::Material, Ray};

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
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + Vector::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        *scattered = Ray::new(record.p, scatter_direction, Some(ray_in.time()));
        *attenuation = self.albedo;

        true
    }
}
