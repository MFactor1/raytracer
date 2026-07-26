pub mod lambertian;
pub mod random;
pub mod metal;
pub mod dielectric;

use rand::Rng;

use crate::objects::Object;

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

pub trait Material: Sync + Send {
    fn scatter<R: Rng, O: Object>(&self, incident: &Ray, normal: &Ray, rng: &mut R, obj: &O) -> Option<ScatterRay>;
}
