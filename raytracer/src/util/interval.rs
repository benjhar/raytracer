use std::ops::Add;

#[must_use]
#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Creates an interval tightly enclosing the two input intervals
    pub fn enclosing(a: Self, b: Self) -> Self {
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };
        Self { min, max }
    }

    #[must_use]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    #[must_use]
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    #[must_use]
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }

    pub const fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.;
        let mut new = *self;
        new.min -= padding;
        new.max += padding;
        new
    }

    #[must_use]
    pub const fn size(&self) -> f64 {
        (self.max - self.min).abs()
    }

    pub const fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub const fn universe() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::empty()
    }
}

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.min + rhs, self.max + rhs)
    }
}
