use crate::{aabb::Aabb, interval::Interval, objects::{Bbox, IntersectableContainer, Object, ObjectSet}, ray::Ray};


pub struct BvhNode {
    left: Option<Box<BvhNode>>,
    right: Option<Box<BvhNode>>,
    obj: Option<Box<dyn Object>>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(mut objs: Vec<Box<dyn Object>>) -> Self {
        let axis: usize = rand::random_range(0..3);

        if objs.len() == 1 {
            let obj = objs.pop().unwrap();
            let bbox = obj.bounding_box().clone();
            return Self {
                left: None,
                right: None,
                obj: Some(obj),
                bbox
            }
        }

        objs.sort_by(|a, b| {
            a.axis_median(axis).total_cmp(&b.axis_median(axis))
        });

        let mid = objs.len() / 2;
        let right = BvhNode::new(objs.split_off(mid));
        let left = BvhNode::new(objs);
        let bbox = Aabb::enclose(&left.bbox, &right.bbox);

        Self {
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            obj: None,
            bbox: bbox,
        }
    }

    pub fn from_objset(set: ObjectSet) -> Self {
        Self::new(set.objs)
    }
}

impl IntersectableContainer for BvhNode {
    fn find_hit(&self, ray: &Ray, interval: &Interval) -> Option<(f64, &Box<dyn Object>)> {
        if !self.bbox.hit(ray, interval) {
            return None
        }

        if let Some(obj) = &self.obj {
            if let Some(ti) = obj.intersects(ray, interval) {
                return Some((ti, obj))
            }
        }

        let mut hit_result = None;

        if let Some(left) = &self.left {
            if let Some(hit) = left.find_hit(ray, interval) {
                hit_result = Some(hit);
            }
        }

        // If left had a hit, only check right for t >= left_hit (i.e. in front of left hit)
        let check_interval_right = if let Some(hit_left) = hit_result {
            &Interval::new(interval.min, hit_left.0)
        } else {
            interval
        };

        if let Some(right) = &self.right {
            if let Some(hit) = right.find_hit(ray, check_interval_right) {
                hit_result = Some(hit);
            }
        }

        hit_result
    }
}

impl Bbox for BvhNode {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
