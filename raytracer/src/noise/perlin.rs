use std::array;

use linalg::{vector::Vector, Point};
use rand::{rngs::StdRng, Rng, SeedableRng};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vector<f64, 3>; POINT_COUNT],
    perm_x: [u32; POINT_COUNT],
    perm_y: [u32; POINT_COUNT],
    perm_z: [u32; POINT_COUNT],
}

impl Perlin {
    pub fn new(seed: Option<u64>) -> Self {
        let mut rng = seed.map_or_else(StdRng::from_entropy, StdRng::seed_from_u64);
        let randvec = array::from_fn(|_| Vector::random_range(-1., 1.).unit());

        let perm_x = Self::permute(
            array::from_fn(|i| u32::try_from(i).unwrap_or_default()),
            &mut rng,
        );
        let perm_y = Self::permute(
            array::from_fn(|i| u32::try_from(i).unwrap_or_default()),
            &mut rng,
        );
        let perm_z = Self::permute(
            array::from_fn(|i| u32::try_from(i).unwrap_or_default()),
            &mut rng,
        );

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    #[expect(clippy::arithmetic_side_effects, reason = "Float")]
    fn value(&self, point: Point<f64, 3>) -> f64 {
        let point_floor = point.map(f64::floor);
        let offset_vector = point - point_floor;
        let [point_i, point_j, point_k] = point_floor.map(|a| a as usize).to_array();

        let mut c = [[[Vector::zero(); 2]; 2]; 2];

        for (di, noise2d) in c.iter_mut().enumerate() {
            for (dj, noise1d) in noise2d.iter_mut().enumerate() {
                #[expect(clippy::indexing_slicing, reason = "All indices within range")]
                for (dk, noise0d) in noise1d.iter_mut().enumerate() {
                    *noise0d = self.randvec[usize::try_from(
                        self.perm_x[(point_i + di) & (POINT_COUNT - 1)]
                            ^ self.perm_y[(point_j + dj) & (POINT_COUNT - 1)]
                            ^ self.perm_z[(point_k + dk) & (POINT_COUNT - 1)],
                    )
                    .unwrap_or_default()];
                }
            }
        }

        Self::perlin_interp(c, offset_vector)
    }

    #[must_use]
    pub fn noise(&self, p: Point<f64, 3>, octaves: usize, roughness: f64, lacunarity: f64) -> f64 {
        let mut acc = 0.;
        let mut temp_p = p;
        let mut weight = 1.;

        #[expect(clippy::arithmetic_side_effects, reason = "Floats")]
        for _ in 0..octaves {
            acc += weight * self.value(temp_p);
            weight *= roughness;
            temp_p = lacunarity * temp_p;
        }

        acc.abs()
    }

    fn permute(mut p: [u32; POINT_COUNT], rng: &mut StdRng) -> [u32; POINT_COUNT] {
        for i in (0..(POINT_COUNT - 1)).rev() {
            let target = rng.gen_range(0..=i);
            p.swap(i, target);
        }

        p
    }

    fn perlin_interp(c: [[[Vector<f64, 3>; 2]; 2]; 2], offset_vector: Vector<f64, 3>) -> f64 {
        let hermite_offset = Self::hermite_cubic(offset_vector);
        let one = Vector::one();
        let mut acc = 0.;
        for (i, plane) in c.iter().enumerate() {
            for (j, row) in plane.iter().enumerate() {
                for (k, cell) in row.iter().enumerate() {
                    let ijk = Vector::new([i as f64, j as f64, k as f64]);
                    let weight_v = offset_vector - ijk;
                    acc += ((ijk * hermite_offset) + (one - ijk) * (one - hermite_offset))
                        .product()
                        * cell.dot(&weight_v);
                }
            }
        }

        acc
    }

    fn hermite_cubic(v: Vector<f64, 3>) -> Vector<f64, 3> {
        v * v * (Vector::new([3.; 3]) - 2. * v)
    }
}
