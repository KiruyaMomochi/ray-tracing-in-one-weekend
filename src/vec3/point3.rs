use super::{Float, Vec3};
use rand::Rng;

pub type Point3 = super::Vec3<f64>;

impl Point3 {
    /// Generate a random point in a unit radius sphere centered at the origin.
    ///
    /// The generation uses the rejection method.
    /// First pick a random point in a unit cube, then reject it if
    /// it is outside the unit sphere.
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.norm() < 1.0 {
                return v;
            }
        }
    }

    /// Generate a random point inside unit hemisphere of the given normal,
    /// centered at the origin.
    pub fn random_in_unit_hemisphere(normal: Vec3<f64>) -> Point3 {
        let v = Self::random_in_unit_sphere();
        if v.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            v
        } else {
            -v
        }
    }

    /// Generate a random point inside unit disk on the XY plane,
    /// centered at the origin.
    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let v = Self::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if v.norm() < 1.0 {
                return v;
            }
        }
    }

    /// Generate a random point in a disk of `radius` centered at the origin.
    pub fn random_in_disk(radius: f64) -> Self {
        if radius <= Float::EPSILON {
            return Self::zeros();
        }

        let mut rng = rand::thread_rng();
        let range = -radius..radius;

        loop {
            let v = Self::new(
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone()),
                0.0,
            );
            if v.norm() < 1.0 {
                return v * radius;
            }
        }
    }
}
