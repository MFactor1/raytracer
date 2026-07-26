pub mod lambertian;
pub mod random;
pub mod metal;
pub mod dielectric;

use rand::Rng;

use crate::objects::{Hit};

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
    fn scatter<R: Rng>(&self, incident: &Ray, hit: &Hit, rng: &mut R) -> Option<ScatterRay>;
}
