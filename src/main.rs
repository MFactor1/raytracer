use pathtracer_lib::vec3::Point3;
use pathtracer_lib::write_img;

fn main() {
    colog::init();
    write_img(3840, 16.0 / 9.0, 2.0, 1.0, &Point3::new(0.0, 0.0, 0.0)).unwrap();
}
