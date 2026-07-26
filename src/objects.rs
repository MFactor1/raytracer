pub mod sphere;
pub mod quad;

use rand::rngs::SmallRng;

use crate::aabb::Aabb;
use crate::vec3::Vec3;

use super::interval::Interval;
use super::ray::Ray;
use super::materials::ScatterRay;

pub struct Hit {
    pub normal: Ray,
    pub t: f64,
    pub u: f64,
    pub v: f64,
}

impl Hit {
    pub fn new(normal: Ray, t: f64, u: f64, v: f64) -> Self {
        Self { normal, t, u, v }
    }
}

pub trait Object: Intersectable + AxisComparable + Scatter + Bbox + Send + Sync {}

pub trait Intersectable {
    /// Gets the t value of the first intersection point, if there exists one.
    fn intersects(&self, ray: &Ray, interval: &Interval) -> Option<Hit>;
}

pub trait IntersectableContainer {
    fn find_hit(&self, ray: &Ray, interval: &Interval) -> Option<(Hit, &Box<dyn Object>)>;
}

pub trait AxisComparable {
    fn axis_median(&self, axis: usize) -> f64;
}

pub trait Scatter {
    fn scatter(&self, incident: &Ray, hit: &Hit, rng: &mut SmallRng) -> Option<ScatterRay>;
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
    fn find_hit(&self, ray: &Ray, interval: &Interval) -> Option<(Hit, &Box<dyn Object>)> {
        let mut ret = None;
        let mut range = interval.clone();
        for obj in self.objs.iter() {
            if let Some(hit) = obj.intersects(ray, &range) {
                range.max = hit.t;
                ret = Some((hit, obj));
            }
        }

        ret
    }
}

#[inline]
pub fn orient_normal(incident: &Ray, normal: Vec3<f64>) -> Vec3<f64> {
    let front_face = incident.direction().dot(normal) < 0.;
    if front_face { normal } else { -normal }
}
