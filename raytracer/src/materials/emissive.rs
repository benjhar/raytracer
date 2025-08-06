use std::sync::Arc;

use crate::{
    materials::Material,
    textures::{Solid, Texture},
    util::colour::Colour,
};

pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

unsafe impl Send for DiffuseLight {}
unsafe impl Sync for DiffuseLight {}

impl DiffuseLight {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_colour(colour: Colour) -> Self {
        Self {
            texture: Arc::new(Solid::from_colour(colour)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, point: &linalg::Point<f64, 3>) -> Colour {
        self.texture.value(u, v, point)
    }
}
