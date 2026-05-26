use std::fs::File;
use std::io::{BufWriter, Write};

use rand::distr::{Uniform, Distribution as _};
use indicatif::ProgressBar;
use log;

use super::color::Color;
use super::interval::Interval;
use super::objects::ObjectSet;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

pub struct Camera {
    /// Num horizontal pixels
    pub frame_width: usize,
    /// Num vertical pixels
    pub frame_height: usize,
    /// Target aspect ratio
    pub aspect_ratio: f64,
    /// Viewport width
    pub vp_width: f64,
    /// Viewport height
    pub vp_height: f64,
    /// Vector along viewport width
    pub vp_u: Vec3<f64>,
    /// Vector along viewport height
    pub vp_v: Vec3<f64>,
    /// Horizonal offset between pixels
    pub pix_delta_u: Vec3<f64>,
    /// Vertical offset between pixels
    pub pix_delta_v: Vec3<f64>,
    /// Point of the top left corner of the viewport
    pub vp_upper_left: Point3<f64>,
    /// Point of the top left pixel in the viewport
    pub pix_00: Point3<f64>,
    /// Distance of camera from viewport
    pub focal_length: f64,
    /// Point of the center of the camera
    pub camera_center: Point3<f64>,
    /// Number of samples to take per pixel
    pub pix_samples: usize,
    /// Maximum number of ray bounces into the scene
    pub max_ray_bounces: usize,
    /// Randomizer used within Camera
    pub rng: rand::rngs::SmallRng,
}

impl Camera {
    pub fn new(
        frame_width: usize,
        aspect_ratio: f64,
        vp_height: f64,
        focal_length: f64,
        camera_center: Point3<f64>,
        pix_samples: usize,
        max_ray_bounces: usize,
    ) -> Self {
        let frame_height = (frame_width as f64 / aspect_ratio) as usize;
        let vp_width = (vp_height * (frame_width as f64 / frame_height as f64)).max(1.0);
        let vp_u = Vec3::new(vp_width, 0.0, 0.0);
        let vp_v = Vec3::new(0.0, -vp_height, 0.0);
        let pix_delta_u = vp_u / frame_width as f64;
        let pix_delta_v = vp_v / frame_height as f64;
        let vp_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - (vp_u + vp_v) * 0.5;
        let pix_00 = vp_upper_left + (pix_delta_u + pix_delta_v) * 0.5;
        let rng = rand::make_rng();

        Self {
            frame_width,
            frame_height,
            aspect_ratio,
            vp_width,
            vp_height,
            vp_u,
            vp_v,
            pix_delta_u,
            pix_delta_v,
            vp_upper_left,
            pix_00,
            focal_length,
            camera_center,
            pix_samples,
            max_ray_bounces,
            rng,
        }
    }

    pub fn render(&mut self, world: &ObjectSet, file: &str) -> std::io::Result<()> {

        let file = File::create(file)?;
        let mut writer = BufWriter::new(file);

        writer.write_all(b"P3\n")?;
        writer.write_all(self.frame_width.to_string().as_bytes())?;
        writer.write_all(b"\n ")?;
        writer.write_all(self.frame_height.to_string().as_bytes())?;
        writer.write_all(b"\n255\n")?;

        let bar = ProgressBar::new((self.frame_height * self.frame_width * self.pix_samples) as u64);

        for j in 0..self.frame_height {
            bar.inc((self.frame_width * self.pix_samples) as u64);
            for i in 0..self.frame_width {
                let pix = self.pix_00 + self.pix_delta_u * i as f64 + self.pix_delta_v * j as f64;
                log::debug!("Pixel: {:?}", pix);

                let mut pix_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.pix_samples {
                    let ray = self.get_ray(i, j);
                    pix_color += self.ray_color(&ray, world, 0);
                }
                pix_color /= self.pix_samples as f64;
                pix_color.write_color(&mut writer)?;
            }
        }
        writer.flush()?;
        Ok(())
    }

    /// Gets a Ray from the camera to a random location inside the given pixel.
    fn get_ray(&mut self, x: usize, y: usize) -> Ray {
        let offset = self.sample_square();
        let pixel_loc = self.pix_00
            + self.pix_delta_u * (x as f64 + offset.x())
            + self.pix_delta_v * (y as f64 + offset.y());

        Ray::new(self.camera_center, pixel_loc - self.camera_center)
    }

    fn sample_square(&mut self) -> Vec3<f64> {
        let dist = Uniform::new(0.0, 0.999).unwrap();
        return Vec3::new(dist.sample(&mut self.rng) - 0.5, dist.sample(&mut self.rng) - 0.5, 0.0);
    }

    fn ray_color(&mut self, ray: &Ray, world: &ObjectSet, depth: usize) -> Color {
        if depth >= self.max_ray_bounces {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some((t, obj)) = world.intersects(ray, &Interval::new(0.001, f64::INFINITY)) {
            let point = ray.at(t);
            let norm = *obj.normal(point).direction();
            // Lambertian diffuse method
            let direction = Vec3::random_unit_vector(&mut self.rng) + norm;
            // Random diffuse method
            //let direction = Vec3::random_on_normal(&mut self.rng, norm);
            return self.ray_color(&Ray::new(point, direction), world, depth + 1) * 0.5;
        }

        let y = ray.direction().unit().y();
        let scale = 0.5 * (y as f64 + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - scale) + Color::new(0.5, 0.7, 1.0) * scale
    }
}


