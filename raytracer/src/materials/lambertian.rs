use std::sync::Arc;

use linalg::vector::Vector;

use crate::{
    engine::{hittable::HitRecord, ray::Ray},
    textures::{Solid, Texture},
    thread_rng,
    util::colour::Colour,
};

use super::Material;

#[derive(Clone)]
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

unsafe impl Sync for Lambertian {}

impl Lambertian {
    #[must_use]
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    #[must_use]
    pub fn from_colour(colour: Colour) -> Self {
        Self {
            texture: Arc::new(Solid::from_colour(colour)),
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
        #[expect(clippy::arithmetic_side_effects, reason = "Floats")]
        let mut scatter_direction = record.normal
            + Vector::random_unit_vector_with_rng(|range| thread_rng().f64_range(range));

        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }

        *scattered = Ray::new(record.p, scatter_direction, Some(ray_in.time()));
        *attenuation = self.texture.value(record.u, record.v, &record.p);

        true
    }
}
