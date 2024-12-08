use crate::{colour::Colour, perlin::Perlin, texture::Texture};

pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(None),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            noise: Perlin::new(Some(seed)),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &linalg::Point<f64, 3>) -> crate::colour::Colour {
        Colour::new([1.; 3]) * self.noise.noise(*p)
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new()
    }
}
