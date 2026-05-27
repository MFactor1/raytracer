use std::rc::Rc;

use pathtracer_lib::camera::Camera;
use pathtracer_lib::materials::dielectric::Dielectric;
use pathtracer_lib::materials::metal::Metal;
use pathtracer_lib::objects::ObjectSet;
use pathtracer_lib::objects::sphere::Sphere;
use pathtracer_lib::materials::lambertian::Lambertian;
use pathtracer_lib::vec3::Point3;
use pathtracer_lib::color::Color;

fn main() {
    colog::init();

    let yellow_lamb = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let blue_lamb = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let grey_metal = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let yellow_metal = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    let clear_glass = Rc::new(Dielectric::new(Color::new(1.0, 0.85, 0.85), 1.5));

    let mut world = ObjectSet::new();
    world.push(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, clear_glass.clone()));
    world.push(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, yellow_metal.clone()));
    world.push(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, blue_lamb.clone()));
    world.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, yellow_lamb.clone()));

    let mut camera = Camera::new(
    1920,
        16.0 / 9.0,
        2.0,
        1.0,
        Point3::new(0.0, 0.0, 0.0),
        50,
        50,
    );

    camera.render(&world, "output.ppm").unwrap();
}
