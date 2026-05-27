pub mod sphere;

use rand::rngs::SmallRng;

use super::interval::Interval;
use super::ray::Ray;
use super::vec3::Point3;
use super::materials::ScatterRay;

pub trait Object: Intersectable + Normal + Scatter {}

pub trait Intersectable {
    /// Gets the t value of the first intersection point, if there exists one.
    fn intersects(&self, ray: &Ray, interval: Interval) -> Option<f64>;
}

pub trait Normal {
    /// Gets the normal vector of the object at the given point.
    /// Assumes the given point lies on the surface of the object. If it does not, the returned
    /// normal vector will be invalid.
    fn normal(&self, point: Point3<f64>) -> Ray;
}

pub trait Scatter {
    fn scatter(&self, incident: &Ray, point: Point3<f64>, rng: &mut SmallRng) -> Option<ScatterRay>;
}

pub struct ObjectSet(Vec<Box<dyn Object>>);

impl ObjectSet {
    #[inline]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn push<Obj: Object + 'static>(&mut self, obj: Obj) {
        self.0.push(Box::new(obj))
    }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    #[inline]
    pub fn intersects(&self, ray: &Ray, interval: Interval) -> Option<(f64, &Box<dyn Object>)> {
        let mut hit = None;
        let mut range = interval.clone();
        for obj in self.0.iter() {
            if let Some(t) = obj.intersects(ray, range) {
                hit = Some((t, obj));
                range.max = t
            }
        }

        hit
    }
}
