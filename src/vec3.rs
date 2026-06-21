use core::ops;
use num_traits::float::Float as FloatBase;
use std::io::{BufWriter, Write};
use std::string::ToString;
use rand::Rng;
use rand::distr::{Uniform, Distribution as _};

pub trait Float:
    FloatBase + ops::AddAssign + ops::SubAssign + ops::MulAssign + ops::DivAssign + ToString + std::fmt::Debug
{
}
impl Float for f32 {}
impl Float for f64 {}

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T: Float>([T; 3]);

impl<T: Float> ops::Index<usize> for Vec3<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: Float> ops::IndexMut<usize> for Vec3<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Float> ops::AddAssign for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl<T: Float> ops::Add for Vec3<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl<T: Float> ops::SubAssign for Vec3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl<T: Float> ops::Sub for Vec3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl<T: Float> ops::Sub<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        Vec3::new(self[0] - rhs, self[1] - rhs, self[2] - rhs)
    }
}

impl ops::Sub<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    #[inline]
    fn sub(self, rhs: Vec3<f64>) -> Vec3<f64> {
        Vec3::new(self - rhs[0], self - rhs[1], self - rhs[2])
    }
}

impl<T: Float> ops::MulAssign<T> for Vec3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl<T: Float> ops::Mul for Vec3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl<T: Float> ops::Mul<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        Vec3::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl<T: Float> ops::DivAssign<T> for Vec3<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

impl<T: Float> ops::Div<T> for Vec3<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        Vec3::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl<T: Float> ops::Neg for Vec3<T> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3::new(-self[0], -self[1], -self[2])
    }
}

impl<T: Float> ToString for Vec3<T> {
    #[inline]
    fn to_string(&self) -> String {
        self[0].to_string() + " " + &self[1].to_string() + " " + &self[2].to_string() + "\n"
    }
}

impl Vec3<f64> {
    #[inline]
    pub fn random<R: Rng>(rng: &mut R, dist: Uniform<f64>) -> Self {
        Vec3([dist.sample(rng), dist.sample(rng), dist.sample(rng)])
    }

    #[inline]
    pub fn random_unit_vector<R: Rng>(rng: &mut R) -> Self {
        let dist = Uniform::new(-1.0, 1.0).unwrap();
        loop {
            let vec = Self::random(rng, dist);
            let length_sq = vec.length_squared();
            if 1e-160 < length_sq && length_sq <= 1.0 {
                return vec / length_sq.sqrt();
            }
        }
    }

    #[inline]
    pub fn random_on_normal<R: Rng>(rng: &mut R, norm: Vec3<f64>) -> Self {
        let vec = Self::random_unit_vector(rng);

        if vec.dot(norm) > 0.0 {
            vec
        } else {
            -vec
        }
    }

    #[inline]
    pub fn random_on_unit_disk<R: Rng>(rng: &mut R) -> Self {
        let dist = Uniform::new(-1.0, 1.0).unwrap();
        loop {
            let p = Vec3::new(dist.sample(rng), dist.sample(rng), 0.0);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }
}

impl<T: Float> Vec3<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3([x, y, z])
    }

    #[inline]
    pub fn to_unit(self) -> Self {
        let len = self.length();
        self / len
    }

    #[inline]
    pub fn unit(&self) -> Self {
        let len = self.length();
        *self / len
    }

    /// Useful because it is equivaent to self.dot(self)
    #[inline]
    pub fn length_squared(&self) -> T {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    #[inline]
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn write<W: Write>(&self, out: &mut BufWriter<W>) -> Result<(), std::io::Error> {
        out.write_all(self.to_string().as_bytes())
    }

    #[inline]
    pub fn dot(&self, rhs: Self) -> T {
        return self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2];
    }

    #[inline]
    pub fn cross(&self, rhs: Self) -> Self {
        Vec3::new(
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        )
    }

    #[inline]
    pub fn x(&self) -> T {
        self[0]
    }

    #[inline]
    pub fn y(&self) -> T {
        self[1]
    }

    #[inline]
    pub fn z(&self) -> T {
        self[2]
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        let e = T::from(1e-8).unwrap();
        self[0].abs() < e && self[1].abs() < e && self[2].abs() < e
    }

    /// Reflect an incident vector according to a given normal unit vector
    #[inline]
    pub fn reflect(&self, normal: Self) -> Self {
        *self - normal * self.dot(normal) * T::from(2).unwrap()
    }

    /// Refract an incident vector according to a given normal unit vector and refractive index
    #[inline]
    pub fn refract(&self, normal: Self, refr_idx: T) -> Self {
        let incident = self.to_unit();
        let cos_theta = (-incident).dot(normal).min(T::from(1.0).unwrap());
        let out_perpendicular = (incident + normal * cos_theta) * refr_idx;
        let out_parallel = normal * (-(((T::from(1.0).unwrap() - out_perpendicular.length_squared()).abs()).sqrt()));
        out_perpendicular + out_parallel
    }
}

pub type Point3<T> = Vec3<T>;
