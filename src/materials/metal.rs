use rand::Rng;

use crate::color::Color;
use crate::materials::ggx::GGX;
use crate::objects::Hit;
use crate::ray::Ray;
use crate::vec3::Vec3;
use super::Material;
use super::ScatterRay;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self { albedo, fuzz: fuzz.clamp(0., 1.) }
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

pub struct GgxMetal {
    pub albedo: Color,
    ggx: GGX,
}

impl GgxMetal {
    pub fn new(albedo: Color, roughness: f32) -> Self {
        Self { albedo, ggx: GGX::new(roughness) }
    }
}

impl Material for GgxMetal {
    fn scatter<R: Rng>(&self, incident: &Ray, hit: &Hit, rng: &mut R) -> Option<ScatterRay> {
        let ggx_scatter = self.ggx.scatter(incident, hit, rng);

        if let Some(scatter) = ggx_scatter {
            let attenuation = self.albedo + (Color::new(1., 1., 1.) - self.albedo) * scatter.fresnel;
            return Some(ScatterRay::new(scatter.ray, attenuation));
        }

        None
    }
}
