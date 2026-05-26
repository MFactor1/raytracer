use pathtracer_lib::camera::Camera;
use pathtracer_lib::objects::ObjectSet;
use pathtracer_lib::objects::sphere::Sphere;
use pathtracer_lib::vec3::Point3;

fn main() {
    colog::init();

    let mut world = ObjectSet::new();
    world.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let mut camera = Camera::new(
        1920,
        16.0 / 9.0,
        2.0,
        1.0,
        Point3::new(0.0, 0.0, 0.0),
        100,
        50,
    );

    camera.render(&world, "output.ppm").unwrap();
}
