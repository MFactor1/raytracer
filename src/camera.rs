use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use rand::distr::{Uniform, Distribution as _};
use rand::Rng;
use indicatif::ProgressBar;
use log;
use rand::rngs::SmallRng;

use crate::bvh::{BvhNode};

use super::color::Color;
use super::interval::Interval;
use super::objects::{IntersectableContainer as _};
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Camera {
    /// Num horizontal pixels
    frame_width: usize,
    /// Num vertical pixels
    frame_height: usize,
    /// Horizonal offset between pixels
    pix_delta_u: Vec3<f64>,
    /// Vertical offset between pixels
    pix_delta_v: Vec3<f64>,
    /// Point of the top left pixel in the viewport
    pix_00: Point3<f64>,
    /// Point of the center of the camera
    camera_center: Point3<f64>,
    /// Number of samples to take per pixel
    pix_samples: usize,
    /// Maximum number of ray bounces into the scene
    max_ray_bounces: usize,
    /// Variation angle of rays through each pixel
    defocus_angle: f64,
    // Defocus disk horizontal radius
    defocus_disk_u: Vec3<f64>,
    // Defocus disk vertical radius
    defocus_disk_v: Vec3<f64>,
    // Scene background color
    background: Color,
}

impl Camera {
    pub fn new(
        frame_width: usize,
        aspect_ratio: f64,
        camera_center: Point3<f64>,
        look_at: Point3<f64>,
        vup: Vec3<f64>,
        pix_samples: usize,
        max_ray_bounces: usize,
        vfov: f64,
        defocus_angle: f64,
        focus_dist: f64,
        background: Color,
    ) -> Self {
        let look_vec = camera_center - look_at;
        let w = look_vec.unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let frame_height = (frame_width as f64 / aspect_ratio) as usize;
        let frame_height = if frame_width < 1 { 1 } else { frame_height };
        let view_height = (vfov.to_radians() / 2.0).tan();
        let vp_height = 2.0 * view_height * focus_dist;
        let vp_width = (vp_height * (frame_width as f64 / frame_height as f64)).max(1.0);

        let vp_u = u * vp_width;
        let vp_v = -v * vp_height;

        let pix_delta_u = vp_u / frame_width as f64;
        let pix_delta_v = vp_v / frame_height as f64;
        let vp_upper_left = camera_center - (w * focus_dist) - vp_u / 2.0 - vp_v / 2.0;
        let pix_00 = vp_upper_left + (pix_delta_u + pix_delta_v) * 0.5;

        let defocus_radius = focus_dist * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            frame_width,
            frame_height,
            pix_delta_u,
            pix_delta_v,
            pix_00,
            camera_center,
            pix_samples,
            max_ray_bounces,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            background,
        }
    }

    pub fn render(&mut self, world: BvhNode, file: &str) -> std::io::Result<()> {
        const NUM_WORKERS: usize = 16;
        let file = File::create(file)?;
        let mut writer = BufWriter::new(file);

        writer.write_all(b"P3\n")?;
        writer.write_all(self.frame_width.to_string().as_bytes())?;
        writer.write_all(b"\n ")?;
        writer.write_all(self.frame_height.to_string().as_bytes())?;
        writer.write_all(b"\n255\n")?;

        let bar = ProgressBar::new((self.frame_height * self.frame_width * self.pix_samples) as u64);

        let (in_tx, in_rx) = mpsc::channel::<usize>();
        let (out_tx, out_rx) = mpsc::channel::<(usize, Vec<Color>)>();
        let in_rx = Arc::new(Mutex::new(in_rx));
        let mut workers = Vec::with_capacity(NUM_WORKERS);
        let world = Arc::new(world);
        let cam = Arc::new(self.clone());

        for _ in 0..NUM_WORKERS {
            let in_rx = in_rx.clone();
            let out_tx = out_tx.clone();
            let world = world.clone();
            let cam = cam.clone();
            let mut rng = rand::make_rng();

            workers.push(thread::spawn(move || {
                loop {
                    let line = {
                        let rx = in_rx.lock().unwrap();
                        rx.recv()
                    };

                    match line {
                        Ok(line) => {
                            let result = cam.process_line(&world, line, &mut rng);
                            out_tx.send((line, result)).unwrap();
                        }
                        Err(_) => break, // channel closed
                    }
                }
            }));
        }

        drop(out_tx);

        for j in 0..self.frame_height {
            in_tx.send(j).unwrap();
        }

        drop(in_tx);

        let mut next = 0;
        let mut pending = BTreeMap::new();

        while let Ok((index, result)) = out_rx.recv() {
            bar.inc((self.frame_width * self.pix_samples) as u64);
            pending.insert(index, result);

            while let Some(result) = pending.remove(&next) {
                for pix in result {
                    pix.write_color(&mut writer)?;
                }
                next += 1;
            }
        }

        writer.flush()?;

        for worker in workers {
            worker.join().unwrap();
        }
        Ok(())
    }

    /// Process one line of pixels, and return the result
    fn process_line(self, world: &BvhNode, line: usize, rng: &mut SmallRng) -> Vec<Color> {
        let mut pixels = Vec::with_capacity(self.frame_width);
        for i in 0..self.frame_width {
            let pix = self.pix_00 + self.pix_delta_u * i as f64 + self.pix_delta_v * line as f64;
            log::debug!("Pixel: {:?}", pix);

            let mut pix_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..self.pix_samples {
                let ray = self.get_ray(i, line, rng);
                pix_color += self.ray_color(ray, world, 0, rng);
            }
            pix_color /= self.pix_samples as f64;
            pixels.push(pix_color);
        }

        pixels
    }

    /// Gets a non-randomized ray, useful for debugging
    #[allow(dead_code)]
    fn get_static_ray(&self, x: usize, y: usize) -> Ray {
        let pixel_loc = self.pix_00
            + self.pix_delta_u * x as f64
            + self.pix_delta_v * y as f64;

        Ray::new(self.camera_center, pixel_loc - self.camera_center)
    }

    /// Gets a Ray from the camera to a random location inside the given pixel.
    fn get_ray<R: Rng>(&self, x: usize, y: usize, rng: &mut R) -> Ray {
        let offset = self.sample_square(rng);
        let pixel_loc = self.pix_00
            + self.pix_delta_u * (x as f64 + offset.x())
            + self.pix_delta_v * (y as f64 + offset.y());

        let ray_origin = if self.defocus_angle <= 0. { self.camera_center } else { self.defocus_disk_sample(rng) };
        Ray::new(ray_origin, pixel_loc - ray_origin)
    }

    fn sample_square<R: Rng>(&self, rng: &mut R) -> Vec3<f64> {
        let dist = Uniform::new(0.0, 0.999).unwrap();
        Vec3::new(dist.sample(rng) - 0.5, dist.sample(rng) - 0.5, 0.0)
    }

    fn defocus_disk_sample<R: Rng>(&self, rng: &mut R) -> Point3<f64> {
        let p = Point3::random_on_unit_disk(rng);
        self.camera_center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y())
    }

    fn ray_color(&self, ray: Ray, world: &BvhNode, depth: usize, rng: &mut SmallRng) -> Color {
        if depth >= self.max_ray_bounces {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some((hit, obj)) = world.find_hit(&ray, &Interval::new(0.001, f64::INFINITY)) {
            let emitted = obj.emit(&hit);
            if let Some(scatter_ray) = obj.scatter(&ray, &hit, rng) {
                return self.ray_color(scatter_ray.ray, world, depth + 1, rng) * scatter_ray.attenuation + emitted;
            } else {
                return emitted;
            }
        }

        self.background
    }
}
