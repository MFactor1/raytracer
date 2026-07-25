pub mod sphere;

use rand::rngs::SmallRng;

use crate::aabb::Aabb;

use super::interval::Interval;
use super::ray::Ray;
use super::vec3::Point3;
use super::materials::ScatterRay;

pub trait Object: Intersectable + AxisComparable + Normal + Scatter + Bbox + Send + Sync {}

pub trait Intersectable {
    /// Gets the t value of the first intersection point, if there exists one.
    fn intersects(&self, ray: &Ray, interval: &Interval) -> Option<f64>;
}

pub trait IntersectableContainer {
    fn find_hit(&self, ray: &Ray, interval: &Interval) -> Option<(f64, &Box<dyn Object>)>;
}

pub trait AxisComparable {
    fn axis_median(&self, axis: usize) -> f64;
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

pub trait Bbox {
    fn bounding_box(&self) -> &Aabb;
}

pub struct ObjectSet {
    pub objs: Vec<Box<dyn Object>>,
    bbox: Option<Aabb>,
}

impl ObjectSet {
    #[inline]
    pub fn new() -> Self {
        Self { objs: Vec::new(), bbox: None}
    }

    #[inline]
    pub fn push<Obj: Object + 'static>(&mut self, obj: Obj) {
        if let Some(bbox) = &self.bbox {
            self.bbox = Some(Aabb::enclose(&bbox, obj.bounding_box()))
        } else {
            self.bbox = Some(obj.bounding_box().clone());
        }

        self.objs.push(Box::new(obj));
    }

    #[inline]
    pub fn clear(&mut self) {
        self.objs.clear()
    }
}

impl IntersectableContainer for ObjectSet{
    #[inline]
    fn find_hit(&self, ray: &Ray, interval: &Interval) -> Option<(f64, &Box<dyn Object>)> {
        let mut hit = None;
        let mut range = interval.clone();
        for obj in self.objs.iter() {
            if let Some(t) = obj.intersects(ray, &range) {
                hit = Some((t, obj));
                range.max = t
            }
        }

        hit
    }
}
