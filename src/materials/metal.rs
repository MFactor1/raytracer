use rand::Rng;

use crate::color::Color;
use crate::objects::Hit;
use crate::ray::Ray;
use crate::vec3::Vec3;
use super::Material;
use super::ScatterRay;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz: fuzz.clamp(0.0, 1.0) }
    }
}

impl Material for Metal {
    fn scatter<R: Rng>(&self, incident: &Ray, hit: &Hit, rng: &mut R) -> Option<ScatterRay> {
        let reflected = incident.direction().reflect(hit.normal.direction());
        let direction = (reflected + (Vec3::random_unit_vector(rng) * self.fuzz)).unit();

        if direction.dot(hit.normal.direction()) > 0.0 {
            Some(ScatterRay::new(Ray::new(hit.normal.origin(), direction), self.albedo))
        } else {
            None
        }
    }
}
