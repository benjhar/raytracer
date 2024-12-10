use crate::{colour::Colour, perlin::Perlin, texture::Texture};

pub struct NoiseTexture {
    noise: Perlin,
    amplitude: f64,
    scale: f64,
    octaves: usize,
    roughness: f64,
    lacunarity: f64,
}

impl NoiseTexture {
    pub fn new(
        amplitude: f64,
        scale: f64,
        octaves: usize,
        roughness: f64,
        lacunarity: f64,
    ) -> Self {
        Self {
            noise: Perlin::new(None),
            amplitude,
            scale,
            octaves,
            roughness,
            lacunarity,
        }
    }

    pub fn with_seed(
        seed: u64,
        amplitude: f64,
        scale: f64,
        octaves: usize,
        roughness: f64,
        lacunarity: f64,
    ) -> Self {
        Self {
            amplitude,
            noise: Perlin::new(Some(seed)),
            scale,
            octaves,
            roughness,
            lacunarity,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: &linalg::Point<f64, 3>) -> crate::colour::Colour {
        Colour::new([self.amplitude; 3])
            * self.noise.noise(
                self.scale * *p,
                self.octaves,
                self.roughness,
                self.lacunarity,
            )
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new(1., 1., 2, 0.5, 2.)
    }
}
