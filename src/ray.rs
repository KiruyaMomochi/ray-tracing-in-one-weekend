use crate::{vec3::{Point3, Vec3}};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3<f64> {
        self.direction
    }

    /// Return a point along the ray at `t`.
    /// Computed by (origin + t * direction)
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
