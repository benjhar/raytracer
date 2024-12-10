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
        let mut rng = if let Some(s) = seed {
            StdRng::seed_from_u64(s)
        } else {
            StdRng::from_entropy()
        };
        let randvec = array::from_fn(|_| Vector::random_range(-1., 1.).unit());

        let perm_x = Self::permute(array::from_fn(|i| i as u32), &mut rng);
        let perm_y = Self::permute(array::from_fn(|i| i as u32), &mut rng);
        let perm_z = Self::permute(array::from_fn(|i| i as u32), &mut rng);

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point<f64, 3>) -> f64 {
        let p_floor = p.map(f64::floor);
        let offset_vector = p - p_floor;
        let [i, j, k] = p_floor.map(|a| a as i64).to_array();

        let mut c = [[[Vector::zero(); 2]; 2]; 2];

        for di in 0..2i64 {
            for dj in 0..2i64 {
                for dk in 0..2i64 {
                    c[di as usize][dj as usize][dk as usize] = self.randvec[(self.perm_x
                        [(i + di) as usize & 255]
                        ^ self.perm_y[(j + dj) as usize & 255]
                        ^ self.perm_z[(k + dk) as usize & 255])
                        as usize];
                }
            }
        }

        Self::perlin_interp(c, offset_vector)
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
