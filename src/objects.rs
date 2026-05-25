use super::ray::Ray;
use super::vec3::Point3;

pub trait Intersectable {
    /// Gets the t value of the first intersection point, if there exists one.
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64>;
}

pub trait Normal {
    /// Gets the normal vector of the object at the given point.
    /// Assumes the given point lies on the surface of the object. If it does not, the returned
    /// normal vector will be invalid.
    fn normal(&self, point: Point3<f64>) -> Ray;
}

pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3<f64>, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
        let o_to_c = self.center - *ray.origin();
        let a = ray.direction().length_squared();
        // b = -2h: allows for a simplification
        let h = ray.direction().dot(o_to_c);
        let c = o_to_c.length_squared() - self.radius.powi(2);
        let descriminant = h.powi(2) - a * c;

        if descriminant < 0.0 {
            return None;
        }

        // We assume the point closest to the camera is the intersetion point we want, so we try to
        // take the smaller of the two values of t (neg descriminant).
        // Simplified quadratic equation due to h simplification.
        let t_1 = (h - descriminant.sqrt()) / a;
        if (t_min..t_max).contains(&t_1) {
            return Some(t_1);
        }

        let t_2 = (h + descriminant.sqrt()) / a;
        if (t_min..t_max).contains(&t_2) {
            return Some(t_2);
        }

        None
    }
}

impl Normal for Sphere {
    fn normal(&self, point: Point3<f64>) -> Ray {
        // We divide the direction by self.radius to normalize the direction vector, since normal
        // vectors must be unit vectors. Assuming the point lies on the sphere, dividing by the
        // radius is a cheap way to normalize, avoiding a square root.
        Ray::new(point, (point - self.center) / self.radius)
    }
}
