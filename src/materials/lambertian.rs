use std::sync::Arc;

use linalg::vector::Vector;

use crate::{
    engine::{hittable::HitRecord, ray::Ray},
    textures::{Solid, Texture},
    util::colour::Colour,
};

use super::Material;

#[derive(Clone)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

unsafe impl Send for Lambertian {}
unsafe impl Sync for Lambertian {}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_colour(colour: Colour) -> Self {
        Self {
            texture: Arc::new(Solid::new(colour)),
        }
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
        *attenuation = self.texture.value(record.u, record.v, &record.p);

        true
    }
}
