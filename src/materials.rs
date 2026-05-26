pub mod lambertian;

use rand::Rng;

use super::ray::Ray;
use super::color::Color;

pub struct ScatterRay {
    pub ray: Ray,
    pub attenuation: Color
}

impl ScatterRay {
    pub fn new(ray: Ray, attenuation: Color) -> Self {
        Self { ray, attenuation }
    }
}

pub trait Material {
    fn scatter<R: Rng>(&self, incident: &Ray, normal: &Ray, rng: &mut R) -> Option<ScatterRay>;
}
