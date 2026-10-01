use crate::{aabb::Aabb, interval::Interval, objects::{Bbox, Hit, Intersectable, Object}, ray::Ray, vec3::{Vec3}};

pub struct Translate<O: Intersectable> {
    inner: O,
    bbox: Aabb,
    offset: Vec3<f32>,
}

impl<O: Intersectable> Translate<O> {
    pub fn new(object: O, offset: Vec3<f32>) -> Translate<O> {
        Self {
            bbox: object.bounding_box() + offset,
            inner: object,
            offset,
        }
    }
}

impl<O: Intersectable> Intersectable for Translate<O> {
    fn intersects(&self, ray: &Ray, interval: &Interval) -> Option<(Hit, &dyn Object)> {
        let offset_ray = Ray::new(ray.origin() - self.offset, ray.direction());

        if let Some(mut hit) = self.inner.intersects(&offset_ray, interval) {
            *hit.0.normal.origin_mut() += self.offset;
            return Some(hit)
        }

        None
    }
}

impl<O: Intersectable> Bbox for Translate<O> {
    #[inline]
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
