use rand::Rng;

use crate::color::Color;
use crate::objects::Object;
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
    fn scatter<R: Rng, O: Object>(&self, incident: &Ray, normal: &Ray, rng: &mut R, _obj: &O) -> Option<ScatterRay> {
        let reflected = incident.direction().reflect(normal.direction());
        let direction = (reflected + (Vec3::random_unit_vector(rng) * self.fuzz)).unit();

        if direction.dot(normal.direction()) > 0.0 {
            Some(ScatterRay::new(Ray::new(normal.origin(), direction), self.albedo))
        } else {
            None
        }
    }
}
