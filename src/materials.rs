pub mod lambertian;
pub mod random;
pub mod metal;
pub mod dielectric;
pub mod emmisive;

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
    #[allow(unused)]
    fn scatter<R: Rng>(&self, incident: &Ray, hit: &Hit, rng: &mut R) -> Option<ScatterRay> {
        None
    }

    #[allow(unused)]
    fn emit(&self, hit: &Hit) -> Color {
        Color::new(0., 0., 0.)
    }
}
