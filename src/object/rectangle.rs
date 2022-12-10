use std::sync::Arc;

use crate::{Hit, HitRecord, Material, Vec3, hit::{AABB, OutwardHitRecord}};

pub struct XYRectangle {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z: f64,
    material: Arc<dyn Material>,
}

impl XYRectangle {
    pub fn new(
        (x0, y0): (f64, f64),
        (x1, y1): (f64, f64),
        z: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            z,
            material,
        }
    }
}

impl Hit for XYRectangle {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // for a ray P(t) = A + t b,
        // where A is the origin and b is the direction,
        // the intersection with the plane z = k is
        let t = (self.z - ray.origin().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return None;
        }

        // find x and y of the intersection point
        let point = ray.at(t);
        let x = point.x();
        let y = point.y();
        // check if the intersection point is inside the rectangle
        if x < self.x0 || x > self.x1 {
            return None;
        }
        if y < self.y0 || y > self.y1 {
            return None;
        }

        // find surface coordinates
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        // the outward normal is always a unit vector along the z axis
        let normal_outward = Vec3::new(0.0, 0.0, 1.0);

        Some(
            OutwardHitRecord::new(point, ray, normal_outward, t, self.material.clone(), (u, v))
                .into_against_ray(),
        )
    }

    fn bounding_box(&self, _time_from: f64, _time_too: f64) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        let padding = f64::EPSILON;
        Some(AABB::new(
            Vec3::new(self.x0, self.x1, self.z - padding),
            Vec3::new(self.y0, self.y1, self.z + padding),
        ))
    }
}
