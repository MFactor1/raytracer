use std::sync::Arc;

use rand::Rng;

use crate::color::Color;
use crate::objects::Object;
use crate::ray::Ray;
use crate::texture::SolidColor;
use crate::texture::Texture;
use crate::vec3::Vec3;
use super::Material;
use super::ScatterRay;

pub struct Lambertian {
    texture: Arc<dyn Texture>
}

impl Lambertian {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self::new(Arc::new(SolidColor::new(albedo)))
    }
}

impl Material for Lambertian {
    fn scatter<R: Rng, O: Object>(&self, _incident: &Ray, normal: &Ray, rng: &mut R, obj: &O) -> Option<ScatterRay> {
        // Lambertian diffuse method
        let mut direction = Vec3::random_unit_vector(rng) + normal.direction();

        // Catch degenerate near-zero scatter directions that may underflow on square root
        if direction.near_zero() {
            direction = normal.direction();
        }

        let (u, v) = obj.get_uv(normal.origin());
        Some(ScatterRay::new(Ray::new(normal.origin(), direction), self.texture.value(u, v, &normal.origin())))
    }
}
