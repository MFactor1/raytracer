use std::sync::Arc;

use crate::{aabb::Aabb, interval::Interval, materials::{Material}, objects::{Bbox, HasMaterial, Hit, Intersectable, Object, ObjectSet}, ray::Ray, vec3::{Point3, Vec3}};

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
    fn intersects(&self, ray: &Ray, interval: &Interval) -> Option<(Hit, &dyn Object)> {
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

        Some((Hit::new(normal, t, alpha, beta), self))
    }
}

impl<M: Material> Bbox for Quad<M> {
    #[inline]
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl<M: Material> HasMaterial for Quad<M> {
    type Mat = M;

    #[inline]
    fn get_material(&self) -> &M {
        &self.material
    }
}

pub fn make_box<M: Material + 'static>(a: Point3<f64>, b: Point3<f64>, material: Arc<M>) -> ObjectSet {
    let mut sides = ObjectSet::new();

    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));
    let dx = Vec3::new(max.x() - min.x(), 0., 0.);
    let dy = Vec3::new(0., max.y() - min.y(), 0.);
    let dz = Vec3::new(0., 0., max.z() - min.z());

    sides.push(Quad::new(min, dx, dy, material.clone())); // front
    sides.push(Quad::new(min, dz, dy, material.clone())); // right
    sides.push(Quad::new(min, dx, dz, material.clone())); // bottom
    sides.push(Quad::new(max, -dx, -dy, material.clone())); // back
    sides.push(Quad::new(max, -dz, -dy, material.clone())); // left
    sides.push(Quad::new(max, -dx, -dz, material.clone())); // top
    sides
}
