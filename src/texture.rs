use std::{path::Path, sync::Arc};

use image::{ImageBuffer, Rgb};
use num_traits::Zero;

use crate::{color::Color, perlin::Perlin, vec3::Point3};

pub trait Texture: Send + Sync {
    /// Get the color value of a given texture at a given texture coordiate,
    /// and 3d position.
    fn value(&self, u: f32, v: f32, p: Point3<f32>) -> Color;
}

pub struct SolidColor {
    albedo: Color
}

impl SolidColor {
    #[inline]
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    #[inline]
    pub fn from_rbg(red: f32, green: f32, blue: f32) -> Self {
        Self::new(Color::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    #[inline]
    fn value(&self, _u: f32, _v: f32, _p: Point3<f32>) -> Color {
        self.albedo
    }
}

pub struct Checkered {
    inv_scale: f32,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checkered {
    #[inline]
    pub fn new(scale: f32, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { inv_scale: 1. / scale, even, odd }
    }

    #[inline]
    pub fn from_color(scale: f32, even: Color, odd: Color) -> Self {
        Self::new(scale, Arc::new(SolidColor::new(even)), Arc::new(SolidColor::new(odd)))
    }
}

impl Texture for Checkered {
    fn value(&self, u: f32, v: f32, p: Point3<f32>) -> Color {
        let xi = (self.inv_scale * p.x()).floor() as i64;
        let yi = (self.inv_scale * p.y()).floor() as i64;
        let zi = (self.inv_scale * p.z()).floor() as i64;

        if ((xi + yi + zi) % 2).is_zero() {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct Image {
    buf: ImageBuffer<Rgb<u8>, Vec<u8>>
}

impl Image {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>
    {
        let mut reader = image::ImageReader::open(path).unwrap();
        reader.no_limits();
        Self { buf: reader.decode().unwrap().to_rgb8() }
    }
}

impl Texture for Image {
    fn value(&self, u: f32, v: f32, _p: Point3<f32>) -> Color {
        if self.buf.height() <= 0 { return Color::new(0., 1., 1.) }

        // Clamp input texture coordiates to [0, 1] x [1, 0]
        let uc = u.clamp(0., 1.);
        let vc = 1. - v.clamp(0., 1.); // Flip V to image coordiates

        let i = (uc * (self.buf.width() - 1) as f32) as u32;
        let j = (vc * (self.buf.height() - 1) as f32) as u32;
        let pixel = self.buf.get_pixel(i, j);
        let color_scale = 1. / 255.;
        Color::new(color_scale * pixel[0] as f32, color_scale * pixel[1] as f32, color_scale * pixel[2] as f32)
    }
}

pub struct Noise {
    noise: Perlin,
    scale: f32,
}

impl Noise {
    pub fn new(scale: f32) -> Self {
        Self { noise: Perlin::new() , scale }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f32, _v: f32, p: Point3<f32>) -> Color {
        Color::new(0.5, 0.5, 0.5) * (1. + (self.scale * p.z() + 10. * self.noise.terbulence(&p, 7)).sin())
    }
}
