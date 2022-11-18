use crate::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Vec3<f64> {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}
