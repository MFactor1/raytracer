use core::f64;
use std::sync::Arc;

use pathtracer_lib::bvh::BvhNode;
use pathtracer_lib::camera::Camera;
use pathtracer_lib::materials::dielectric::Dielectric;
use pathtracer_lib::materials::metal::Metal;
use pathtracer_lib::objects::ObjectSet;
use pathtracer_lib::objects::sphere::Sphere;
use pathtracer_lib::materials::lambertian::Lambertian;
use pathtracer_lib::texture::Checkered;
use pathtracer_lib::texture::Image;
use pathtracer_lib::vec3::Point3;
use pathtracer_lib::vec3::Vec3;
use pathtracer_lib::color::Color;
use rand::distr::{Uniform, Distribution as _};

fn book_1_demo() {
    colog::init();
    let mut world = ObjectSet::new();
    let checker = Arc::new(Checkered::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let ground_material = Arc::new(Lambertian::new(checker.clone()));
    world.push(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    let dist = Uniform::new(0.0, 1.0).unwrap();
    let dist_high = Uniform::new(0.5, 1.0).unwrap();
    let dist_low = Uniform::new(0., 0.5).unwrap();
    let dist_lowlow = Uniform::new(0., 0.05).unwrap();
    let mut rng: rand::rngs::SmallRng = rand::make_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = dist.sample(&mut rng);
            let center = Point3::new(a as f64 + 0.9*dist.sample(&mut rng), 0.2, b as f64 + 0.9*dist.sample(&mut rng));

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random(&mut rng, dist) * Color::random(&mut rng, dist);
                    world.push(Sphere::new(center, 0.2, Arc::new(Lambertian::from_color(albedo))));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(&mut rng, dist_high);
                    let fuzz = dist_low.sample(&mut rng);
                    world.push(Sphere::new(center, 0.2, Arc::new(Metal::new(albedo, fuzz))));
                } else {
                    let fuzz = dist_lowlow.sample(&mut rng);
                    world.push(Sphere::new(center, 0.2, Arc::new(Dielectric::new(Color::new(1., 1., 1.), 1.5, fuzz))));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(Color::new(1., 1., 1.), 1.5, 0.));
    world.push(Sphere::new(Point3::new(0., 1., 0.), 1., mat1));
    let mat2 = Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    world.push(Sphere::new(Point3::new(-4., 1., 0.), 1., mat2));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.push(Sphere::new(Point3::new(4., 1., 0.), 1., mat3));

    let bounded_world = BvhNode::from_objset(world);

    let mut camera = Camera::new(
    3440,
        21.0 / 9.0,
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        50,
        50,
        20.0,
        0.6,
        10.,
    );

    camera.render(bounded_world, "output.ppm").unwrap();
}

fn checkered_spheres() {
    colog::init();
    let mut world = ObjectSet::new();
    let checker = Arc::new(Checkered::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
    let ground_material = Arc::new(Lambertian::new(checker.clone()));
    world.push(Sphere::new(Point3::new(0.0, -10., 0.0), 10.0, ground_material.clone()));
    world.push(Sphere::new(Point3::new(0.0, 10., 0.0), 10.0, ground_material.clone()));

    let bounded_world = BvhNode::from_objset(world);

    let mut camera = Camera::new(
    3440,
        21.0 / 9.0,
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        50,
        50,
        20.0,
        0.,
        10.,
    );

    camera.render(bounded_world, "output.ppm").unwrap();
}

fn earth() {
    colog::init();
    let mut world = ObjectSet::new();
    let earth_tex = Arc::new(Image::new("assets/earthmap_sat.jpg"));
    let earth_mat = Arc::new(Lambertian::new(earth_tex.clone()));
    let globe = Sphere::new(Point3::new(0., 0., 0.), 2., earth_mat.clone());
    world.push(globe);

    let bounded_world = BvhNode::from_objset(world);

    let mut camera = Camera::new(
    3440,
        21.0 / 9.0,
        Point3::new(0., 0., 12.),
        Point3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        50,
        50,
        20.0,
        0.,
        10.,
    );

    camera.render(bounded_world, "output.ppm").unwrap();
}

fn main() {
    match 3 {
        1 => book_1_demo(),
        2 => checkered_spheres(),
        3 => earth(),
        _ => panic!("Not an option"),
    }
}
