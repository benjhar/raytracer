use std::array;

use linalg::Point;
use rand::{rngs::StdRng, Rng, SeedableRng};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [u32; POINT_COUNT],
    perm_y: [u32; POINT_COUNT],
    perm_z: [u32; POINT_COUNT],
}

impl Perlin {
    pub fn new(seed: Option<u64>) -> Self {
        let mut rng = if let Some(s) = seed {
            StdRng::seed_from_u64(s)
        } else {
            StdRng::from_entropy()
        };
        let randfloat: [f64; POINT_COUNT] = array::from_fn(|_| rng.gen());

        let perm_x = Self::permute(array::from_fn(|i| i as u32), &mut rng);
        let perm_y = Self::permute(array::from_fn(|i| i as u32), &mut rng);
        let perm_z = Self::permute(array::from_fn(|i| i as u32), &mut rng);

        Self {
            randfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point<f64, 3>) -> f64 {
        let a: [f64; 3] = (4. * p).into();
        let [i, j, k] = a.map(|x| x.abs() as usize & 255);

        self.randfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }

    fn permute(mut p: [u32; POINT_COUNT], rng: &mut StdRng) -> [u32; POINT_COUNT] {
        for i in (0..(POINT_COUNT - 1)).rev() {
            let target = rng.gen_range(0..=i);
            p.swap(i, target);
        }

        p
    }
}
