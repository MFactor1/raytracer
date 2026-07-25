use std::sync::Arc;

use rand::rngs::SmallRng;

use super::{Intersectable, Normal, Scatter, Object};
use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::objects::{AxisComparable, Bbox};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::materials::{Material, ScatterRay};

impl<M: Material + Send + Sync> Object for Sphere<M> {}

pub struct Sphere<M: Material> {
    center: Point3<f64>,
    radius: f64,
    material: Arc<M>,
    bbox: Aabb,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3<f64>, radius: f64, material: Arc<M>) -> Self {
        // The bbox is just constructed using the vector of size r, r, r,
        // and using center - v, center + v as the extrema. This guarentees
        // the sphere fits exactly in the bbox.
        let radius_vector = Vec3::new(radius, radius, radius);
        let bbox = Aabb::from_extrema(center - radius_vector, center + radius_vector);

        Self { center, radius, material, bbox }
    }
}

impl<M: Material> Intersectable for Sphere<M> {
    fn intersects(&self, ray: &Ray, interval: &Interval) -> Option<f64> {
        let o_to_c = self.center - ray.origin();
        let a = ray.direction().length_squared();
        // b = -2h: allows for a simplification
        let h = ray.direction().dot(o_to_c);
        let c = o_to_c.length_squared() - self.radius.powi(2);
        let descriminant = h.powi(2) - a * c;

        if descriminant < 0.0 {
            return None;
        }

        // We assume the point closest to the camera is the intersetion point we want, so we try to
        // take the smaller of the two values of t (neg descriminant).
        // Simplified quadratic equation due to h simplification.
        let t_1 = (h - descriminant.sqrt()) / a;
        if interval.contains(&t_1) {
            return Some(t_1);
        }

        let t_2 = (h + descriminant.sqrt()) / a;
        if interval.contains(&t_2) {
            return Some(t_2);
        }

        None
    }
}

impl<M: Material> Normal for Sphere<M> {
    #[inline]
    fn normal(&self, point: Point3<f64>) -> Ray {
        // We divide the direction by self.radius to normalize the direction vector, since normal
        // vectors must be unit vectors. Assuming the point lies on the sphere, dividing by the
        // radius is a cheap way to normalize, avoiding a square root.
        Ray::new(point, (point - self.center) / self.radius)
    }
}

impl<M: Material> Scatter for Sphere<M> {
    #[inline]
    fn scatter(&self, incident: &Ray, point: Point3<f64>, rng: &mut SmallRng) -> Option<ScatterRay> {
        self.material.scatter(incident, &self.normal(point), rng)
    }
}

impl<M: Material> Bbox for Sphere<M> {
    #[inline]
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl<M: Material> AxisComparable for Sphere<M> {
    #[inline]
    fn axis_median(&self, axis: usize) -> f64 {
        self.bounding_box().get_axis(axis).median()
    }
}
