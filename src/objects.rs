pub mod sphere;
pub mod quad;
pub mod rotate;
pub mod rect;
pub mod translate;

use rand::rngs::SmallRng;

use crate::aabb::Aabb;
use crate::color::Color;
use crate::materials::Material;
use crate::vec3::Vec3;

use super::interval::Interval;
use super::ray::Ray;
use super::materials::ScatterRay;

pub struct Hit {
    pub normal: Ray,
    pub t: f32,
    pub u: f32,
    pub v: f32,
}

impl Hit {
    pub fn new(normal: Ray, t: f32, u: f32, v: f32) -> Self {
        Self { normal, t, u, v }
    }
}

pub trait Object: Intersectable + AxisComparable + Scatter + Bbox + Emmisive + Send + Sync {}


pub trait Intersectable: Bbox + AxisComparable + Send + Sync {
    /// Gets the t value of the first intersection point, if there exists one.
    fn intersects(&self, ray: &Ray, interval: &Interval) -> Option<(Hit, &dyn Object)>;
}

pub trait AxisComparable {
    fn axis_median(&self, axis: usize) -> f32;
}

impl<T: Bbox> AxisComparable for T {
    fn axis_median(&self, axis: usize) -> f32 {
        self.bounding_box().get_axis(axis).median()
    }
}

pub trait HasMaterial {
    type Mat: Material;
    fn get_material(&self) -> &Self::Mat;
}

pub trait Scatter {
    fn scatter(&self, incident: &Ray, hit: &Hit, rng: &mut SmallRng) -> Option<ScatterRay>;
}

impl<T: HasMaterial> Scatter for T {
    #[inline]
    fn scatter(&self, incident: &Ray, hit: &Hit, rng: &mut SmallRng) -> Option<ScatterRay> {
        self.get_material().scatter(incident, hit, rng)
    }
}

pub trait Bbox {
    fn bounding_box(&self) -> &Aabb;
}

pub trait Emmisive {
    fn emit(&self, hit: &Hit) -> Color;
}

impl<T: HasMaterial> Emmisive for T {
    #[inline]
    fn emit(&self, hit: &Hit) -> Color {
        self.get_material().emit(hit)
    }
}

pub struct ObjectSet {
    pub objs: Vec<Box<dyn Intersectable>>,
    bbox: Aabb,
    empty_bbox: bool,
}

impl ObjectSet {
    #[inline]
    pub fn new() -> Self {
        Self { objs: Vec::new(), bbox: Aabb::default(), empty_bbox: true}
    }

    #[inline]
    pub fn push<Obj: Intersectable + 'static>(&mut self, obj: Obj) {
        if self.empty_bbox {
            self.bbox = obj.bounding_box().clone();
        } else {
            self.bbox = Aabb::enclose(&self.bbox, obj.bounding_box())
        }

        self.objs.push(Box::new(obj));
        self.empty_bbox = false;
    }

    #[inline]
    pub fn clear(&mut self) {
        self.objs.clear()
    }
}

impl Intersectable for ObjectSet {
    #[inline]
    fn intersects(&self, ray: &Ray, interval: &Interval) -> Option<(Hit, &dyn Object)> {
        let mut ret = None;
        let mut range = interval.clone();
        for obj in self.objs.iter() {
            if let Some(hit) = obj.intersects(ray, &range) {
                range.max = hit.0.t;
                ret = Some(hit);
            }
        }

        ret
    }
}

impl Bbox for ObjectSet {
    #[inline]
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

#[inline]
pub fn orient_normal(incident: &Ray, normal: Vec3<f32>) -> Vec3<f32> {
    let front_face = incident.direction().dot(normal) < 0.;
    if front_face { normal } else { -normal }
}
