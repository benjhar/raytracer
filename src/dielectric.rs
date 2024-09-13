use linalg::vector::Vector;

use crate::{colour::Colour, material::Material, Ray};

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
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
        let refracted = Vector::refract(&unit_direction, &record.normal, ri);

        *scattered = Ray::new(record.p, refracted);

        true
    }
}
