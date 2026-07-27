use core::f64;
use std::sync::Arc;

use pathtracer_lib::bvh::BvhNode;
use pathtracer_lib::camera::Camera;
use pathtracer_lib::materials::dielectric::Dielectric;
use pathtracer_lib::materials::emmisive::Emmisive;
use pathtracer_lib::materials::metal::Metal;
use pathtracer_lib::objects::ObjectSet;
use pathtracer_lib::objects::quad::Quad;
use pathtracer_lib::objects::sphere::Sphere;
use pathtracer_lib::materials::lambertian::Lambertian;
use pathtracer_lib::texture::Checkered;
use pathtracer_lib::texture::Image;
use pathtracer_lib::texture::Noise;
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
        Color::new(0.7, 0.8, 1.),
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
        Color::new(0.7, 0.8, 1.),
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
        Color::new(0.7, 0.8, 1.),
    );

    camera.render(bounded_world, "output.ppm").unwrap();
}

fn perlin_spheres() {
    colog::init();
    let mut world = ObjectSet::new();
    let perlin_tex = Arc::new(Noise::new(4.));
    let perlin_mat = Arc::new(Lambertian::new(perlin_tex.clone()));
    world.push(Sphere::new(Point3::new(0., -1000., 0.), 1000., perlin_mat.clone()));
    world.push(Sphere::new(Point3::new(0., 2., 0.), 2., perlin_mat.clone()));

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
        Color::new(0.7, 0.8, 1.),
    );

    camera.render(bounded_world, "output.ppm").unwrap();
}

fn quads() {
    colog::init();
    let mut world = ObjectSet::new();

    let red = Arc::new(Lambertian::from_color(Color::new(1., 0.2, 0.2)));
    let green = Arc::new(Lambertian::from_color(Color::new(0.2, 1., 0.2)));
    let blue = Arc::new(Lambertian::from_color(Color::new(0.2, 0.2, 1.)));
    let orange = Arc::new(Lambertian::from_color(Color::new(1., 0.5, 0.0)));
    let teal = Arc::new(Lambertian::from_color(Color::new(0.2, 0.8, 0.8)));

    world.push(Quad::new(Point3::new(-3., -2., 5.), Vec3::new(0., 0., -4.), Vec3::new(0., 4., 0.), red.clone()));
    world.push(Quad::new(Point3::new(-2., -2., 0.), Vec3::new(4., 0., 0.), Vec3::new(0., 4., 0.), green.clone()));
    world.push(Quad::new(Point3::new(3., -2., 1.), Vec3::new(0., 0., 4.), Vec3::new(0., 4., 0.), blue.clone()));
    world.push(Quad::new(Point3::new(-2., 3., 1.), Vec3::new(4., 0., 0.), Vec3::new(0., 0., 4.), orange.clone()));
    world.push(Quad::new(Point3::new(-2., -3., 5.), Vec3::new(4., 0., 0.), Vec3::new(0., 0., -4.), teal.clone()));

    let bounded_world = BvhNode::from_objset(world);

    let mut camera = Camera::new(
    3440,
        21.0 / 9.0,
        Point3::new(0., 0., 9.),
        Point3::new(0., 0., 0.),
        Vec3::new(0., 1., 0.),
        500,
        50,
        80.0,
        0.,
        10.,
        Color::new(0.7, 0.8, 1.),
    );

    camera.render(bounded_world, "output.ppm").unwrap();
}

fn simple_light() {
    colog::init();
    let mut world = ObjectSet::new();
    let perlin_tex = Arc::new(Noise::new(4.));
    let perlin_mat = Arc::new(Lambertian::new(perlin_tex.clone()));
    world.push(Sphere::new(Point3::new(0., -1000., 0.), 1000., perlin_mat.clone()));
    world.push(Sphere::new(Point3::new(-3., 2., 0.), 2., perlin_mat.clone()));

    let light_red = Arc::new(Emmisive::from_color(Color::new(2., 0.5, 0.5)));
    let light_white_low = Arc::new(Emmisive::from_color(Color::new(1.5, 1.14, 0.795)));
    //let light_white_med = Arc::new(Emmisive::from_color(Color::new(2., 1.52, 1.)));
    let light_white_high = Arc::new(Emmisive::from_color(Color::new(10., 7.6, 5.3)));
    //let light_green = Arc::new(Emmisive::from_color(Color::new(0.5, 2., 0.5)));
    let light_green = Arc::new(Emmisive::from_color(Color::new(0.25, 1., 0.25)));
    let glass = Arc::new(Dielectric::new(Color::new(1., 1., 1.), 1.5, 0.));
    let air = Arc::new(Dielectric::new(Color::new(1., 1., 1.), 1. / 1.5, 0.));
    let matte = Arc::new(Lambertian::from_color(Color::new(0.3, 0.3, 0.3)));
    let matte_dark = Arc::new(Lambertian::from_color(Color::new(0.1, 0.1, 0.1)));
    let metal = Arc::new(Metal::new(Color::new(1., 1., 1.), 0.3));
    let matte_pink = Arc::new(Lambertian::from_color(Color::new(1., 0.08, 0.58)));

    // Top-light and backing
    world.push(Quad::new(Point3::new(5., 3., 0.5), Vec3::new(1.5, 0., 0.), Vec3::new(0., 0.75, -0.75), light_white_high.clone()));
    world.push(Quad::new(Point3::new(5., 3.02, 0.51), Vec3::new(1.5, 0., 0.), Vec3::new(0., 0.75, -0.75), matte.clone()));

    // Top-light balls
    world.push(Sphere::new(Point3::new(6., 1.1, -3.), 0.75, glass.clone()));
    world.push(Sphere::new(Point3::new(6., 0.7, -1.), 0.5, matte_pink.clone()));

    // Green light/ball stack
    world.push(Quad::new(Point3::new(3., 0., 0.), Vec3::new(1.5, 0., 0.), Vec3::new(0., 0., 1.5), light_green.clone()));
    world.push(Sphere::new(Point3::new(3.75, 0.75, 0.75), 0.75, glass.clone()));
    world.push(Sphere::new(Point3::new(3.75, 2., 0.75), 0.5, matte_dark.clone()));

    // Red ball light on left
    world.push(Sphere::new(Point3::new(2., 1.3, 4.), 0.8, light_white_low.clone()));
    world.push(Sphere::new(Point3::new(2., 1.3, 4.), 0.9, air.clone()));
    world.push(Sphere::new(Point3::new(2., 1.3, 4.), 1., glass.clone()));
    world.push(Sphere::new(Point3::new(2.5, 0.5, 5.4), 0.5, metal.clone()));

    // White ball light in back
    world.push(Sphere::new(Point3::new(-0., 1.7, -3.), 0.6, light_red.clone()));
    world.push(Sphere::new(Point3::new(-0., 1.7, -3.), 0.7, air.clone()));
    world.push(Sphere::new(Point3::new(-0., 1.7, -3.), 0.8, glass.clone()));

    let bounded_world = BvhNode::from_objset(world);

    let mut camera = Camera::new(
    3440,
        21.0 / 9.0,
        Point3::new(26., 5., 6.),
        Point3::new(0., 1.5, -1.),
        Vec3::new(0., 1., 0.),
        50000,
        50,
        16.0,
        0.,
        10.,
        Color::new(0.005, 0.005, 0.03),
    );

    camera.render(bounded_world, "output.ppm").unwrap();
}

fn main() {
    match 6 {
        1 => book_1_demo(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        _ => panic!("Not an option"),
    }
}
