pub mod color;
pub mod objects;
pub mod ray;
pub mod vec3;

use color::Color;
use log;
use objects::{Intersectable as _, Normal as _, Sphere};
use ray::Ray;
use std::fs::File;
use std::io::{BufWriter, Write};
use vec3::{Point3, Vec3};

fn ray_color(ray: &Ray) -> Color {
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    if let Some(t) = sphere.intersects(ray, 0.0, f64::MAX) {
        let norm = sphere.normal(ray.at(t));
        return Color::new(
            (norm.direction().x() + 1.0) as f32,
            (norm.direction().y() + 1.0) as f32,
            (norm.direction().z() + 1.0) as f32,
        ) * 0.5;
    }

    let y = ray.direction().unit().y();
    let scale = 0.5 * (y as f32 + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - scale) + Color::new(0.5, 0.7, 1.0) * scale
}

pub fn write_img(
    width: usize,
    ratio: f64,
    vp_height: f64,
    focal_len: f64,
    camera_center: &Point3<f64>,
) -> std::io::Result<()> {
    let height = (width as f64 / ratio) as usize;
    let vp_width = (vp_height * (width as f64 / height as f64)).max(1.0);
    let vp_u = Vec3::new(vp_width, 0.0, 0.0);
    let vp_v = Vec3::new(0.0, -vp_height, 0.0);
    let vp_delta_u = vp_u / width as f64;
    let vp_delta_v = vp_v / height as f64;
    let vp_upper_left = *camera_center - Vec3::new(0.0, 0.0, focal_len) - (vp_u + vp_v) * 0.5;
    let pix_00 = vp_upper_left + (vp_delta_u + vp_delta_v) * 0.5;

    log::debug!("Frame width: {:?}", width);
    log::debug!("Frame height: {:?}", height);
    log::debug!("VP width: {:?}", vp_width);
    log::debug!("VP height: {:?}", vp_height);
    log::debug!("VP U: {:?}", vp_u);
    log::debug!("VP V: {:?}", vp_v);
    log::debug!("VP DU: {:?}", vp_delta_u);
    log::debug!("VP DV: {:?}", vp_delta_v);
    log::debug!("VP upper left: {:?}", vp_upper_left);
    log::debug!("Pixel 00: {:?}", pix_00);

    let file = File::create("output.ppm")?;
    let mut writer = BufWriter::new(file);

    writer.write_all(b"P3\n")?;
    writer.write_all(width.to_string().as_bytes())?;
    writer.write_all(b"\n ")?;
    writer.write_all(height.to_string().as_bytes())?;
    writer.write_all(b"\n255\n")?;

    for j in 0..height {
        log::info!("Lines remaining: {}", height - j);
        for i in 0..width {
            let pix = pix_00 + vp_delta_u * i as f64 + vp_delta_v * j as f64;
            log::debug!("Pixel: {:?}", pix);

            let ray = Ray::new(*camera_center, pix - *camera_center);
            let pix_color = ray_color(&ray);
            pix_color.write_color(&mut writer)?;
        }
    }
    writer.flush()?;
    Ok(())
}
