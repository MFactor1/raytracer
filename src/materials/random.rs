use rand::Rng;

use crate::color::Color;
use crate::objects::Hit;
use crate::ray::Ray;
use crate::vec3::Vec3;
use super::Material;
use super::ScatterRay;

pub struct Random {
    pub albedo: Color
}

impl Random {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Random {
    fn scatter<R: Rng>(&self, _incident: &Ray, hit: &Hit, rng: &mut R) -> Option<ScatterRay> {
        // Random diffuse method
        let direction = Vec3::random_on_normal(rng, hit.normal.direction());

        Some(ScatterRay::new(Ray::new(hit.normal.origin(), direction), self.albedo))
    }
}
