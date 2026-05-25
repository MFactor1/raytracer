use super::vec3::Vec3;
use std::io::{BufWriter, Write};

pub type Color = Vec3<f32>;

impl Color {
    pub fn write_color<W: Write>(&self, out: &mut BufWriter<W>) -> Result<(), std::io::Error> {
        let r = (self[0] * 255.999) as u8;
        let g = (self[1] * 255.999) as u8;
        let b = (self[2] * 255.999) as u8;
        writeln!(out, "{} {} {}", r, g, b)
    }
}
