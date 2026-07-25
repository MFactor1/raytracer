//! AABB (Axis Aligned Bounding Box)

use crate::{ray::Ray, vec3::Point3};

use super::interval::Interval;

#[derive(Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    // Creates Aabb from the two extreme-most corner points
    pub fn from_extrema(a: Point3<f64>, b: Point3<f64>) -> Self {
        let x = if a.x() <= b.x() {
            Interval::new(a.x(), b.x())
        } else {
            Interval::new(b.x(), a.x())
        };
        let y = if a.y() <= b.y() {
            Interval::new(a.y(), b.y())
        } else {
            Interval::new(b.y(), a.y())
        };
        let z = if a.z() <= b.z() {
            Interval::new(a.z(), b.z())
        } else {
            Interval::new(b.z(), a.z())
        };
        Self { x, y, z }
    }

    // Creates an Aabb that perfectly encloses two existing Aabb
    pub fn enclose(a: &Aabb, b: &Aabb) -> Self {
        Self {
            x: Interval::union(&a.x, &b.x),
            y: Interval::union(&a.y, &b.y),
            z: Interval::union(&a.z, &b.z),
        }
    }

    pub fn hit(&self, incident: &Ray, interval: &Interval) -> bool {
        let mut window = interval.clone();
        for idx in 0..3 {
            let (axis, in_dir, in_orig) = match idx {
                0 => (self.x, incident.direction().x(), incident.origin().x()),
                1 => (self.y, incident.direction().y(), incident.origin().y()),
                _ => (self.z, incident.direction().z(), incident.origin().z()),
            };

            let t0 = (axis.min - in_orig) / in_dir;
            let t1 = (axis.max - in_orig) / in_dir;

            if t0 < t1 {
                if t0 > window.min { window.min = t0 }
                if t1 < window.max { window.max = t1 }
            } else {
                if t1 > window.min { window.min = t1 }
                if t0 < window.max { window.max = t0 }
            }

            if window.min >= window.max {
                return false;
            }
        }

        true
    }

    pub fn get_axis(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            _ => &self.z,
        }
    }
}
