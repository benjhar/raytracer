use std::{cell::UnsafeCell, ops::RangeBounds, ops::RangeInclusive, rc::Rc};

use fastrand::Rng;

pub mod bounding_volume_hierarchies;
pub mod engine;
pub mod materials;
pub mod noise;
pub mod surface;
pub mod textures;
pub mod util;

/// A reference to the thread-local generator
///
#[derive(Clone, Debug)]
pub struct ThreadRng {
    // Rc is explicitly !Send and !Sync
    rng: Rc<UnsafeCell<Rng>>,
}

thread_local!(
    static THREAD_RNG_KEY: Rc<UnsafeCell<Rng>> = {
        let rng = Rng::new();
        Rc::new(UnsafeCell::new(rng))
    }
);

pub fn thread_rng() -> ThreadRng {
    let rng = THREAD_RNG_KEY.with(|t| t.clone());
    ThreadRng { rng }
}

impl ThreadRng {
    /// Generates a random `usize` in the given range.
    ///
    /// Panics if the range is empty.
    pub fn usize(&mut self, range: impl RangeBounds<usize>) -> usize {
        let rng = unsafe { &mut *self.rng.get() };
        rng.usize(range)
    }

    /// Generates a random `f64` in range `0..=1`.
    pub fn f64_inclusive(&mut self) -> f64 {
        let rng = unsafe { &mut *self.rng.get() };
        rng.f64_inclusive()
    }

    /// Generates a random `f64` in the given range.
    ///
    /// Panics if the range is empty.
    pub fn f64_range(&mut self, range: RangeInclusive<f64>) -> f64 {
        let start_bound = match range.start_bound() {
            std::ops::Bound::Included(s) => *s,
            std::ops::Bound::Excluded(_) | std::ops::Bound::Unbounded => unreachable!(),
        };
        let end_bound = match range.end_bound() {
            std::ops::Bound::Included(s) => *s,
            std::ops::Bound::Excluded(_) | std::ops::Bound::Unbounded => unreachable!(),
        };
        let range_size = end_bound - start_bound;
        assert!(range_size >= 0.0, "Empty range");
        let rng = unsafe { &mut *self.rng.get() };
        rng.f64_inclusive().mul_add(range_size, start_bound)
    }
}
