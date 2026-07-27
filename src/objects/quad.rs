use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::{aabb::Aabb, color::Color, interval::Interval, materials::{Material, ScatterRay}, objects::{AxisComparable, Bbox, Emmisive, Hit, Intersectable, Object, Scatter}, ray::Ray, vec3::{Point3, Vec3}};

impl<M: Material + Send + Sync> Object for Quad<M> {}

pub struct Quad<M: Material> {
    origin: Point3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    w: Vec3<f64>,
    normal: Vec3<f64>,
    d: f64,
    material: Arc<M>,
    bbox: Aabb,
}

impl<M: Material> Quad<M> {
    pub fn new(origin: Point3<f64>, u: Vec3<f64>, v: Vec3<f64>, material: Arc<M>) -> Self {
        let bbox_diag1 = Aabb::from_extrema(origin, origin + u + v);
        let bbox_diag2 = Aabb::from_extrema(origin + u, origin + v);
        let bbox = Aabb::enclose(&bbox_diag1, &bbox_diag2);
        let n = u.cross(v);
        let normal = n.to_unit();
        let d = normal.dot(origin);
        let w = n / n.dot(n);

        Self { origin, u, v, material, bbox, normal, d, w }
    }

    #[inline]
    fn normal(&self, incident: &Ray, point: Point3<f64>) -> Ray {
        Ray::new(point, super::orient_normal(incident, self.normal))
    }
}

impl<M: Material> Intersectable for Quad<M> {
    fn intersects(&self, ray: &Ray, interval: &Interval) -> Option<Hit> {
        let denominator = self.normal.dot(ray.direction());

        // If the ray is parallel to the plane
        if denominator.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin())) / denominator;
        if !interval.contains(&t) {
            return None;
        }

        let intersection = ray.at(t);
        let planar_hit_vector = intersection - self.origin;
        let alpha = self.w.dot(planar_hit_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_vector));

        if !(0_f64..=1_f64).contains(&alpha) || !(0_f64..=1_f64).contains(&beta) {
            return None
        }

        let normal = self.normal(ray, intersection);

        Some(Hit::new(normal, t, alpha, beta))
    }
}

impl<M: Material> Scatter for Quad<M> {
    #[inline]
    fn scatter(&self, incident: &Ray, hit: &Hit, rng: &mut SmallRng) -> Option<ScatterRay> {
        self.material.scatter(incident, hit, rng)
    }
}

impl<M: Material> Bbox for Quad<M> {
    #[inline]
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl<M: Material> AxisComparable for Quad<M> {
    #[inline]
    fn axis_median(&self, axis: usize) -> f64 {
        self.bbox.get_axis(axis).median()
    }
}

impl<M: Material> Emmisive for Quad<M> {
    #[inline]
    fn emit(&self, hit: &Hit) -> Color {
        self.material.emit(hit)
    }
}
