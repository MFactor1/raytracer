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
    pub fn union(a: &Interval, b: &Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
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

    #[inline]
    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)
    }

    #[inline]
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.;
        Self {
            min: self.min - padding,
            max: self.max + padding
        }
    }

    #[inline]
    pub fn median(&self) -> f64 {
        (self.min + self.max) / 2.
    }

    #[inline]
    pub fn len(&self) -> f64 {
        (self.max - self.min).abs()
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self { min: 0., max: 0. }
    }
}
