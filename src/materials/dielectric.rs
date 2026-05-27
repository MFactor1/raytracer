use rand::Rng;

use crate::color::Color;
use crate::ray::Ray;
use super::Material;
use super::ScatterRay;

pub struct Dielectric {
    pub albedo: Color,
    pub refr_idx: f64
}

impl Dielectric {
    pub fn new(albedo: Color, refr_idx: f64) -> Self {
        Self { albedo, refr_idx: refr_idx }
    }
}

impl Material for Dielectric {
    fn scatter<R: Rng>(&self, incident: &Ray, normal: &Ray, _rng: &mut R) -> Option<ScatterRay> {
        let internal = incident.direction().dot(normal.direction()) > 0.0;
        let ri = if internal { self.refr_idx } else { 1.0 / self.refr_idx };
        let norm = if internal { -normal.direction() } else { normal.direction() };

        let refracted = incident.direction().refract(norm, ri);

        Some(ScatterRay::new(Ray::new(normal.origin(), refracted), self.albedo))
    }
}
