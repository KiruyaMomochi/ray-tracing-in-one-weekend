use crate::{
    hit::{Hit, OutwardHitRecord},
    vec3::{Point3, Vec3},
};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3<f64>,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3<f64>, time: f64) -> Self {
        assert_ne!(direction.len_squared(), 0.0);
        Self { origin, direction, time }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3<f64> {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    /// Return a point along the ray at `t`.
    /// Computed by (origin + t * direction)
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    /// Move the ray origin by the given offset.
    /// This is useful for translating the ray into the object's local space.
    pub fn move_origin_by(self, offset: Vec3<f64>) -> Self {
        Self::new(self.origin + offset, self.direction, self.time)
    }
}

impl Ray {
    pub fn hit<T: Hit>(self, hittable: &T, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        hittable.hit(self, t_min, t_max)
    }
}
