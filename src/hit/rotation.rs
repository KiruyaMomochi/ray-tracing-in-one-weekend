use std::sync::RwLock;

use crate::{Hit, Ray, Vec3};

use super::{OutwardHitRecord, AABB};

#[derive(Debug)]
pub struct Rotate<H: Hit> {
    object: H,
    /// Rotation angle in radians
    angle: f64,
    /// Sine of rotation angle, used for lazy evaluation
    sin: f64,
    /// Cosine of rotation angle, used for lazy evaluation
    cos: f64,
    /// Rotate axis, first index is the axis to rotate around
    axis: [usize; 3],
    /// Time range of bounding box, used for lazy evaluation
    time_range: RwLock<Option<(f64, f64)>>,
    /// Bounding box of the object, lazily evaluated
    bounding_box: RwLock<Option<AABB>>,
}

impl<H: Hit + Clone> Clone for Rotate<H> {
    fn clone(&self) -> Self {
        Self {
            object: self.object.clone(),
            angle: self.angle,
            sin: self.sin,
            cos: self.cos,
            axis: self.axis,
            time_range: RwLock::new(None),
            bounding_box: RwLock::new(None),
        }
    }
}

impl<H: Hit> Rotate<H> {
    fn new(object: H, degree: f64, axis: [usize; 3]) -> Self {
        let angle = degree.to_radians();
        let sin = angle.sin();
        let cos = angle.cos();

        Self {
            object,
            angle,
            sin,
            cos,
            axis,
            time_range: RwLock::new(None),
            bounding_box: RwLock::new(None),
        }
    }

    pub fn new_x(object: H, degree: f64) -> Self {
        Self::new(object, degree, [0, 1, 2])
    }

    pub fn new_y(object: H, degree: f64) -> Self {
        Self::new(object, degree, [1, 0, 2])
    }

    pub fn new_z(object: H, degree: f64) -> Self {
        Self::new(object, degree, [2, 0, 1])
    }

    fn rotate(&self, point: &Vec3<f64>) -> Vec3<f64> {
        let mut vec = Vec3::zeros();
        vec[self.axis[0]] = point[self.axis[0]];
        vec[self.axis[1]] = self.cos * point[self.axis[1]] - self.sin * point[self.axis[2]];
        vec[self.axis[2]] = self.sin * point[self.axis[1]] + self.cos * point[self.axis[2]];
        vec
    }

    fn rotate_inv(&self, point: &Vec3<f64>) -> Vec3<f64> {
        let mut vec = Vec3::zeros();
        vec[self.axis[0]] = point[self.axis[0]];
        vec[self.axis[1]] = self.cos * point[self.axis[1]] + self.sin * point[self.axis[2]];
        vec[self.axis[2]] = -self.sin * point[self.axis[1]] + self.cos * point[self.axis[2]];
        vec
    }
}

impl<H: Hit> Hit for Rotate<H> {
    fn hit(&self, ray: crate::Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        let origin = self.rotate(&ray.origin());
        let direction = self.rotate(&ray.direction());
        let rotated_ray = Ray::new(origin, direction, ray.time());

        rotated_ray.hit(&self.object, t_min, t_max).map(|mut hit| {
            hit.point = self.rotate_inv(&hit.point);
            hit.normal_outward = self.rotate_inv(&hit.normal_outward);
            hit
        })
    }

    fn bounding_box(&self, time_from: f64, time_to: f64) -> Option<AABB> {
        // If the time range is the same as the last time we calculated the bounding box, we can
        // return the cached value.

        if let Some(time_range) = *self.time_range.read().unwrap() {
            if time_range.0 == time_from && time_range.1 == time_to {
                return self.bounding_box.read().unwrap().clone();
            }
        };

        // Otherwise, we need to calculate the bounding box again.
        *self.time_range.write().unwrap() = Some((time_from, time_to));
        *self.bounding_box.write().unwrap() =
            self.object.bounding_box(time_from, time_to).map(|aabb| {
                aabb.into_iter_corners().fold(AABB::EMPTY, |aabb, corner| {
                    aabb.include(&self.rotate(&corner))
                })
            });

        self.bounding_box.read().unwrap().clone()
    }
}
