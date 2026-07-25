use core::f64;
use std::sync::Arc;

use pathtracer_lib::bvh::BvhNode;
use pathtracer_lib::camera::Camera;
use pathtracer_lib::materials::dielectric::Dielectric;
use pathtracer_lib::materials::metal::Metal;
use pathtracer_lib::objects::ObjectSet;
use pathtracer_lib::objects::sphere::Sphere;
use pathtracer_lib::materials::lambertian::Lambertian;
use pathtracer_lib::vec3::Point3;
use pathtracer_lib::vec3::Vec3;
use pathtracer_lib::color::Color;
use rand::distr::{Uniform, Distribution as _};

fn main() {
    colog::init();
    let mut world = ObjectSet::new();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    //let yellow_lamb = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    //let blue_lamb = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    //let grey_metal = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.01));
    //let yellow_metal = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    //let clear_glass = Rc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.5, 0.00));
    //let bubble = Rc::new(Dielectric::new(Color::new(1.0, 1.0, 1.0), 1.0 / 1.5, 0.00));

    //world.push(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, clear_glass.clone()));
    //world.push(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, bubble.clone()));
    //world.push(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, grey_metal.clone()));
    //world.push(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.4, blue_lamb.clone()));
    //world.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, yellow_lamb.clone()));

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
                    world.push(Sphere::new(center, 0.2, Arc::new(Lambertian::new(albedo))));
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
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
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
