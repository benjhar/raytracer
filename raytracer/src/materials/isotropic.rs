use std::sync::Arc;

use linalg::vector::Vector;

use super::Material;
use crate::{
    engine::ray::Ray,
    textures::{Solid, Texture},
    util::colour::Colour,
};

pub struct Isotropic {
    texture: Arc<dyn Texture>,
}

unsafe impl Send for Isotropic {}
unsafe impl Sync for Isotropic {}

impl Isotropic {
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

impl Material for Isotropic {
    fn scatter(
        &self,
        ray_in: &crate::engine::ray::Ray,
        record: &crate::engine::hittable::HitRecord,
        attenuation: &mut Colour,
        scattered: &mut crate::engine::ray::Ray,
    ) -> bool {
        *scattered = Ray::new(record.p, Vector::random_unit_vector(), Some(ray_in.time()));
        *attenuation = self.texture.value(record.u, record.v, &record.p);
        true
    }
}
