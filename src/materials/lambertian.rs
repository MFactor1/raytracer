use std::sync::Arc;

use rand::Rng;

use crate::color::Color;
use crate::materials::ggx::GGX;
use crate::objects::Hit;
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
    fn scatter<R: Rng>(&self, _incident: &Ray, hit: &Hit, rng: &mut R) -> Option<ScatterRay> {
        // Lambertian diffuse method
        let mut direction = Vec3::random_unit_vector(rng) + hit.normal.direction();

        // Catch degenerate near-zero scatter directions that may underflow on square root
        if direction.near_zero() {
            direction = hit.normal.direction();
        }

        Some(ScatterRay::new(Ray::new(hit.normal.origin(), direction), self.texture.value(hit.u, hit.v, hit.normal.origin())))
    }
}

pub struct GgxLambertian {
    texture: Arc<dyn Texture>,
    ggx: GGX,
}

impl GgxLambertian {
    pub fn new(texture: Arc<dyn Texture>, roughness: f32) -> Self {
        Self {texture, ggx: GGX::new(roughness) }
    }

    pub fn from_color(albedo: Color, roughness: f32) -> Self {
        Self::new(Arc::new(SolidColor::new(albedo)), roughness)
    }
}

impl Material for GgxLambertian {
    fn scatter<R: Rng>(&self, incident: &Ray, hit: &Hit, rng: &mut R) -> Option<ScatterRay> {
        let ggx_scatter = self.ggx.scatter(incident, hit, rng);

        if let Some(scatter) = ggx_scatter {
            let mut direction = scatter.ray.direction().unit() + hit.normal.direction();

            if direction.near_zero() {
                direction = hit.normal.direction();
            }

            return Some(ScatterRay::new(Ray::new(hit.normal.origin(), direction), self.texture.value(hit.u, hit.v, hit.normal.origin())));
        }

        None
    }
}
