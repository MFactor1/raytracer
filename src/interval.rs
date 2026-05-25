#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    #[inline]
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    #[inline]
    pub fn contains(&self, x: &f64) -> bool {
        (self.min..=self.max).contains(x)
    }

    #[inline]
    pub fn surrounds(&self, x: &f64) -> bool {
        self.min < *x && *x < self.max
    }
}
