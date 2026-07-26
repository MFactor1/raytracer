use rand::{distr::{Uniform}, seq::SliceRandom};

use crate::vec3::{Point3, Vec3};

pub struct Perlin {
    randvec: [Vec3<f64>; Perlin::POINT_COUNT],
    perm_x: [usize; Perlin::POINT_COUNT],
    perm_y: [usize; Perlin::POINT_COUNT],
    perm_z: [usize; Perlin::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rng = rand::rng();
        let dist = Uniform::new(-1.0, 1.0).unwrap();
        let randvec: [Vec3<f64>; Perlin::POINT_COUNT] = std::array::from_fn(|_| Vec3::random(&mut rng, dist).to_unit());

        Self {
            randvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, point: &Point3<f64>) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();
        let i = point.x().floor() as i64;
        let j = point.y().floor() as i64;
        let k = point.z().floor() as i64;
        let mut c: [[[Vec3<f64>; 2]; 2]; 2] = [[[Vec3::new(0., 0., 0.); 2]; 2]; 2];

        for di in 0_i64..2_i64 {
            for dj in 0_i64..2_i64 {
                for dk in 0_i64..2_i64 {
                    c[di as usize][dj as usize][dk as usize] = self.randvec[
                        self.perm_x[((i + di) & 255) as usize] ^
                        self.perm_y[((j + dj) & 255) as usize] ^
                        self.perm_z[((k + dk) & 255) as usize]
                    ];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn terbulence(&self, point: &Point3<f64>, depth: usize) -> f64 {
        let mut accum = 0.;
        let mut temp_point = point.clone();
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_point);
            weight *= 0.5;
            temp_point *= 2.;
        }

        accum.abs()
    }

    fn perlin_generate_perm() -> [usize; Perlin::POINT_COUNT] {
        let mut out: [usize; Perlin::POINT_COUNT] = std::array::from_fn(|i| i as usize);
        let mut rng = rand::rng();
        out.shuffle(&mut rng);
        out
    }

    fn perlin_interp(c: [[[Vec3<f64>; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);
        let mut accum = 0.;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1. - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1. - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1. - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }

        accum
    }
}
