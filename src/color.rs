use super::vec3::Vec3;
use std::io::{BufWriter, Write};

pub type Color = Vec3<f32>;

impl Color {
    pub fn write_color<W: Write>(&self, out: &mut BufWriter<W>, gamma: f32) -> Result<(), std::io::Error> {
        let r = (linear_to_gamma(self[0], gamma).clamp(0.0, 0.999) * 256.0) as u8;
        let g = (linear_to_gamma(self[1], gamma).clamp(0.0, 0.999) * 256.0) as u8;
        let b = (linear_to_gamma(self[2], gamma).clamp(0.0, 0.999) * 256.0) as u8;
        writeln!(out, "{} {} {}", r, g, b)
    }
}

#[inline]
fn linear_to_gamma(linear: f32, gamma: f32) -> f32 {
    if linear > 0.0 {
        return linear.powf(1. / gamma);
    }

    0.0
}
