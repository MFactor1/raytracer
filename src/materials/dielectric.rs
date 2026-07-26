use rand::Rng;
use rand::distr::Distribution;
use rand::distr::Uniform;

use crate::color::Color;
use crate::objects::Object;
use crate::ray::Ray;
use crate::vec3::Vec3;
use super::Material;
use super::ScatterRay;

pub struct Dielectric {
    pub albedo: Color,
    pub refr_idx: f64,
    pub fuzz: f64,
}

impl Dielectric {
    pub fn new(albedo: Color, refr_idx: f64, fuzz: f64) -> Self {
        Self { albedo, refr_idx, fuzz}
    }
}

impl Material for Dielectric {
    fn scatter<R: Rng, O: Object>(&self, incident: &Ray, normal: &Ray, rng: &mut R, _obj: &O) -> Option<ScatterRay> {
        let internal = incident.direction().dot(normal.direction()) > 0.0;
        let ri = if internal { self.refr_idx } else { 1.0 / self.refr_idx };
        let norm = if internal { -normal.direction() } else { normal.direction() };

        let cos_theta = (-incident.direction().unit()).dot(norm).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction: Vec3<f64>;
        let rand_range = Uniform::new(0.0, 1.0).unwrap();
        let fuzz_vec = Vec3::random_unit_vector(rng) * self.fuzz;

        if ri * sin_theta > 1.0 || reflectance(cos_theta, ri, 1.0) > rand_range.sample(rng) {
            direction = incident.direction().reflect(norm);
        } else {
            direction = incident.direction().refract(norm, ri);
        }

        Some(ScatterRay::new(Ray::new(normal.origin(), (direction + fuzz_vec).unit()), self.albedo))
    }
}

// Schlick's approximation for reflectance
fn reflectance(cosine: f64, eta_i: f64, eta_t: f64) -> f64 {
    let r0 = ((eta_i - eta_t) / (eta_i + eta_t)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
