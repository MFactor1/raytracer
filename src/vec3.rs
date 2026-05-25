use core::ops;
use num_traits::float::Float as FloatBase;
use std::io::{BufWriter, Write};
use std::string::ToString;

pub trait Float:
    FloatBase + ops::AddAssign + ops::SubAssign + ops::MulAssign + ops::DivAssign + ToString
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

impl<T: Float> ToString for Vec3<T> {
    #[inline]
    fn to_string(&self) -> String {
        self[0].to_string() + " " + &self[1].to_string() + " " + &self[2].to_string() + "\n"
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
}

pub type Point3<T> = Vec3<T>;
