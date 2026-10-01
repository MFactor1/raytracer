use super::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    orig: Point3<f32>,
    dir: Vec3<f32>,
}

impl Ray {
    #[inline]
    pub fn new(orig: Point3<f32>, dir: Vec3<f32>) -> Self {
        Self { orig, dir }
    }

    #[inline]
    pub fn origin(&self) -> Vec3<f32> {
        self.orig
    }

    #[inline]
    pub fn origin_mut(&mut self) -> &mut Vec3<f32> {
        &mut self.orig
    }

    #[inline]
    pub fn direction(&self) -> Point3<f32> {
        self.dir
    }

    #[inline]
    pub fn direction_mut(&mut self) -> &mut Vec3<f32> {
        &mut self.dir
    }

    #[inline]
    pub fn at(&self, t: f32) -> Point3<f32> {
        self.orig + self.dir * t
    }
}
