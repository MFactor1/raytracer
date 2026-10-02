use rand::Rng;
use rand_distr::Distribution;
use rand_distr::Uniform;

use crate::objects::Hit;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct GGXScatter {
    pub ray: Ray,
    pub fresnel: f32,
}

impl GGXScatter {
    pub fn new(ray: Ray, fresnel: f32) -> Self {
        Self {
            ray,
            fresnel,
        }
    }
}

/// GGX helper, see https://jcgt.org/published/0007/04/01/
pub struct GGX {
    pub alpha: f32
}

impl GGX {
    pub fn new(roughness: f32) -> Self {
        Self { alpha: roughness.powi(2).max(1e-3) }
    }

    #[inline]
    fn lambda(&self, cos_theta: f32) -> f32 {
        let cos_2 = cos_theta * cos_theta;
        let tan_2 = (1. - cos_2).max(0.) / cos_2;
        ((1.0_f32 + self.alpha * self.alpha * tan_2).sqrt() - 1.) * 0.5
    }

    #[inline]
    fn fresnel(&self, cos_theta: f32) -> f32 {
        (1. - cos_theta).clamp(0., 1.).powi(5)
    }

    fn sample_vndf(&self, view: Vec3<f32>, rand1: f32, rand2: f32) -> Vec3<f32> {
        // Transform from ellipsoid space to hemisphere
        let v_hemis = Vec3::new(self.alpha * view.x(), self.alpha * view.y(), view.z());

        // Construct orthonomal basis
        let lensq = v_hemis.x().powi(2) + v_hemis.y().powi(2);
        let tangent_1 = if lensq > 0. {
            Vec3::new(-v_hemis.y(), v_hemis.x(), 0.) / lensq.sqrt()
        } else {
            Vec3::new(1., 0., 0.)
        };
        let tangent_2 = v_hemis.cross(tangent_1);

        // Generate a uniformly distr point on the unit circle defined by the above tangents.
        // This then maps to a point on the hemisphere.
        let r = rand1.sqrt();
        let phi = 2. * std::f32::consts::PI * rand2;
        let p1 = r * phi.cos();
        let mut p2 = r * phi.sin();
        let s = 0.5 * (1. + v_hemis.z());
        p2 = (1. - s) * (1. - p1.powi(2)).max(0.).sqrt() + s * p2;

        // Project unit circle point back onto hemisphere.
        let norm_hemis = tangent_1 * p1 + tangent_2 * p2 + v_hemis * (1. - p1.powi(2) - p2.powi(2)).max(0.).sqrt();

        // Transform the normal back to the ellipsoid
        Vec3::new(self.alpha * norm_hemis.x(), self.alpha * norm_hemis.y(), norm_hemis.z().max(0.)).unit()
    }

    pub fn scatter<R: Rng>(&self, incident: &Ray, hit: &Hit, rng: &mut R) -> Option<GGXScatter> {
        let hit_norm = hit.normal.direction();
        let v_world = -incident.direction().unit();

        // construct local orthonomal plane to hit_norm where z is up
        let helper = if hit_norm.x().abs() > 0.9 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        };
        let tangent = hit_norm.cross(helper).unit();
        let btangent = hit_norm.cross(tangent);

        // transform a world vector to our local space, and vice-versa
        let to_local = |v: Vec3<f32>| Vec3::new(v.dot(tangent), v.dot(btangent), v.dot(hit_norm));
        let to_world = |v: Vec3<f32>| tangent * v.x() + btangent * v.y() + hit_norm * v.z();

        let v_local = to_local(v_world);

        // If the viewing direction is below the surface, kill the ray.
        if v_local.z() <= 0. { return None; }

        let dist = Uniform::new(0., 1.).unwrap();
        let facet_norm = self.sample_vndf(v_local, dist.sample(rng), dist.sample(rng));

        // reflect() expects v_local to be an incident vector, but we negated it earlier to make it
        // point outwards
        let result_local = (-v_local).reflect(facet_norm);

        // If the reflected ray got refelcted down into the material, kill it.
        if result_local.z() <= 0. { return None; }

        // Calculate lambdas to simulate light getting caught on other ridges/bumps on the way in
        // (v_local) and on the way out (result_local)
        let lambda_v = self.lambda(v_local.z());
        let lambda_result = self.lambda(result_local.z());

        let fresnel_mult = self.fresnel(v_local.dot(facet_norm)) * ((1. + lambda_v) / (1. + lambda_v + lambda_result));
        Some(GGXScatter::new(Ray::new(hit.normal.origin(), to_world(result_local)), fresnel_mult))
    }
}
