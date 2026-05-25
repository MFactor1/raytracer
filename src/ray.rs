use super::vec3::{Point3, Vec3};

#[derive(Debug)]
pub struct Ray {
    orig: Point3<f64>,
    dir: Vec3<f64>,
}

impl Ray {
    pub fn new(orig: Point3<f64>, dir: Vec3<f64>) -> Self {
        Self { orig, dir }
    }

    pub fn origin(&self) -> &Vec3<f64> {
        &self.orig
    }

    pub fn direction(&self) -> &Point3<f64> {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3<f64> {
        self.orig + self.dir * t
    }
}
