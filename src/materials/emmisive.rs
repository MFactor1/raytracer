use std::sync::Arc;

use crate::{color::Color, materials::Material, objects::Hit, texture::{SolidColor, Texture}};

pub struct Emmisive {
    texture: Arc<dyn Texture>,
}

impl Emmisive {
    pub fn new(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }

    pub fn from_color(color: Color) -> Self {
        Self { texture: Arc::new(SolidColor::new(color)) }
    }
}

impl Material for Emmisive {
    fn emit(&self, hit: &Hit) -> Color {
        self.texture.value(hit.u, hit.v, hit.normal.origin())
    }
}
