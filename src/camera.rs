use std::fs::File;
use std::io::{BufWriter, Write};

use indicatif::ProgressBar;
use log;

use super::color::Color;
use super::interval::Interval;
use super::objects::ObjectSet;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

pub struct Camera {
    pub frame_width: usize,
    pub aspect_ratio: f64,
    pub vp_height: f64,
    pub focal_length: f64,
    pub camera_center: Point3<f64>,
}

impl Camera {
    pub fn new(
        frame_width: usize,
        aspect_ratio: f64,
        vp_height: f64,
        focal_length: f64,
        camera_center: Point3<f64>,
    ) -> Self {
        Self {
            frame_width,
            aspect_ratio,
            vp_height,
            focal_length,
            camera_center,
        }
    }

    pub fn render(&self, world: &ObjectSet, file: &str) -> std::io::Result<()> {
        let frame_height = (self.frame_width as f64 / self.aspect_ratio) as usize;
        let vp_width = (self.vp_height * (self.frame_width as f64 / frame_height as f64)).max(1.0);
        let vp_u = Vec3::new(vp_width, 0.0, 0.0);
        let vp_v = Vec3::new(0.0, -self.vp_height, 0.0);
        let vp_delta_u = vp_u / self.frame_width as f64;
        let vp_delta_v = vp_v / frame_height as f64;
        let vp_upper_left =
            self.camera_center - Vec3::new(0.0, 0.0, self.focal_length) - (vp_u + vp_v) * 0.5;
        let pix_00 = vp_upper_left + (vp_delta_u + vp_delta_v) * 0.5;

        let file = File::create(file)?;
        let mut writer = BufWriter::new(file);

        writer.write_all(b"P3\n")?;
        writer.write_all(self.frame_width.to_string().as_bytes())?;
        writer.write_all(b"\n ")?;
        writer.write_all(frame_height.to_string().as_bytes())?;
        writer.write_all(b"\n255\n")?;

        let bar = ProgressBar::new(frame_height as u64);

        for j in 0..frame_height {
            bar.inc(1);
            for i in 0..self.frame_width {
                let pix = pix_00 + vp_delta_u * i as f64 + vp_delta_v * j as f64;
                log::debug!("Pixel: {:?}", pix);

                let ray = Ray::new(self.camera_center, pix - self.camera_center);
                let pix_color = ray_color(&ray, world);
                pix_color.write_color(&mut writer)?;
            }
        }
        writer.flush()?;
        Ok(())
    }
}

fn ray_color(ray: &Ray, world: &ObjectSet) -> Color {
    if let Some((t, obj)) = world.intersects(ray, &Interval::new(0.0, f64::INFINITY)) {
        return (*obj.normal(ray.at(t)).direction() + Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let y = ray.direction().unit().y();
    let scale = 0.5 * (y as f64 + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - scale) + Color::new(0.5, 0.7, 1.0) * scale
}
