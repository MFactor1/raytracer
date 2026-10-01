use itertools::iproduct;

use crate::{aabb::Aabb, interval::Interval, objects::{Bbox, Hit, Intersectable, Object}, ray::Ray, vec3::{Point3, Vec3}};

pub struct RotateY<O: Intersectable> {
    inner: O,
    cos_theta: f64,
    sin_theta: f64,
    bbox: Aabb,
}

impl<O: Intersectable> RotateY<O> {
    pub fn new(object: O, angle: f64) -> RotateY<O> {
        let (sin_theta, cos_theta) = angle.to_radians().sin_cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for (i, j, k) in iproduct!(0..2, 0..2, 0..2).map(|(a, b, c)| (a as f64, b as f64, c as f64)) {
            let x = i * bbox.x().max + (1. - i) * bbox.x().min;
            let y = j * bbox.y().max + (1. - j) * bbox.y().min;
            let z = k * bbox.z().max + (1. - k) * bbox.z().min;

            let new_x = cos_theta * x + sin_theta * z;
            let new_z = cos_theta * z - sin_theta * x;
            let tester = Vec3::new(new_x, y, new_z);

            for c in 0..3 {
                min[c] = min[c].min(tester[c]);
                max[c] = max[c].max(tester[c]);
            }
        }

        let bbox = Aabb::from_extrema(min, max);

        Self {
            inner: object,
            cos_theta,
            sin_theta,
            bbox,
        }
    }
}

impl<O: Intersectable> Intersectable for RotateY<O> {
    fn intersects(&self, ray: &Ray, interval: &Interval) -> Option<(Hit, &dyn Object)> {

        // convert ray to Object space (rotate by -theta)
        let object_ray = Ray::new(
            Point3::new(
                self.cos_theta * ray.origin().x() - self.sin_theta * ray.origin().z(),
                ray.origin().y(),
                self.cos_theta * ray.origin().z() + self.sin_theta * ray.origin().x(),
            ),
            Vec3::new(
                self.cos_theta * ray.direction().x() - self.sin_theta * ray.direction().z(),
                ray.direction().y(),
                self.cos_theta * ray.direction().z() + self.sin_theta * ray.direction().x(),
            )
        );

        if let Some((mut hit, obj)) = self.inner.intersects(&object_ray, interval) {
            // convert normal to real space (rotate by theta)
            hit.normal = Ray::new(
                Point3::new(
                    self.cos_theta * hit.normal.origin().x() + self.sin_theta * hit.normal.origin().z(),
                    hit.normal.origin().y(),
                    self.cos_theta * hit.normal.origin().z() - self.sin_theta * hit.normal.origin().x(),
                ),
                Vec3::new(
                    self.cos_theta * hit.normal.direction().x() + self.sin_theta * hit.normal.direction().z(),
                    hit.normal.direction().y(),
                    self.cos_theta * hit.normal.direction().z() - self.sin_theta * hit.normal.direction().x(),
                )
            );

            return Some((hit, obj))
        }

        None
    }
}

impl<O: Intersectable> Bbox for RotateY<O> {
    #[inline]
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
