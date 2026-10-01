use core::ops;

#[derive(Copy, Clone)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    };
    pub const UNIVERSE: Self = Self {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    };

    #[inline]
    pub fn new(min: f32, max: f32) -> Self {
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
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    #[inline]
    pub fn contains(&self, x: &f32) -> bool {
        (self.min..=self.max).contains(x)
    }

    #[inline]
    pub fn surrounds(&self, x: &f32) -> bool {
        self.min < *x && *x < self.max
    }

    #[inline]
    pub fn clamp(&self, x: f32) -> f32 {
        x.clamp(self.min, self.max)
    }

    #[inline]
    pub fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.;
        Self {
            min: self.min - padding,
            max: self.max + padding
        }
    }

    #[inline]
    pub fn median(&self) -> f32 {
        (self.min + self.max) / 2.
    }

    #[inline]
    pub fn len(&self) -> f32 {
        (self.max - self.min).abs()
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self { min: 0., max: 0. }
    }
}

impl ops::AddAssign<f32> for Interval {
    #[inline]
    fn add_assign(&mut self, val: f32) {
        self.min += val;
        self.max += val;
    }
}

impl ops::Add<f32> for Interval {
    type Output = Self;

    #[inline]
    fn add(self, val: f32) -> Self::Output {
        Interval::new(self.min + val, self.max + val)
    }
}
