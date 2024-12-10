use crate::{colour::Colour, perlin::Perlin, texture::Texture};

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(None),
            scale,
        }
    }

    pub fn with_seed(seed: u64, scale: f64) -> Self {
        Self {
            noise: Perlin::new(Some(seed)),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &linalg::Point<f64, 3>) -> crate::colour::Colour {
        Colour::new([1.; 3]) * 0.5 * (1. + self.noise.noise(self.scale * *p))
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new(1.)
    }
}
