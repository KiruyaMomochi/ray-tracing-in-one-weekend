use crate::Ray;
use std::sync::Arc;

use crate::{
    hit::{OutwardHitRecord, AABB},
    Hit, Material, Vec3,
};

#[derive(Debug, Clone)]
pub struct AxisAlignedRectangle {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    z: f64,
    /// The first index is the axis of the normal,
    /// the following two index is the axis of the plane.
    axis: [usize; 3],
    material: Arc<dyn Material>,
}

impl AxisAlignedRectangle {
    pub fn new(
        min_coord: (f64, f64),
        max_coord: (f64, f64),
        z: f64,
        axis: [usize; 3],
        material: Arc<dyn Material>,
    ) -> Self {
        let (x0, y0) = min_coord;
        let (x1, y1) = max_coord;

        assert!(
            axis[0] != axis[1] && axis[0] != axis[2] && axis[1] != axis[2],
            "axis must be different"
        );
        assert!(
            x0 < x1,
            "axis {}: min coordinate must be less than max",
            axis[1]
        );
        assert!(
            y0 < y1,
            "axis {}: min coordinate must be less than max",
            axis[2]
        );
        Self {
            x0,
            x1,
            y0,
            y1,
            z,
            axis,
            material,
        }
    }

    pub fn new_xy(
        min_coord: (f64, f64),
        max_coord: (f64, f64),
        z: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self::new(min_coord, max_coord, z, [2, 0, 1], material)
    }

    pub fn new_xz(
        min_coord: (f64, f64),
        max_coord: (f64, f64),
        y: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self::new(min_coord, max_coord, y, [1, 0, 2], material)
    }

    pub fn new_yz(
        min_coord: (f64, f64),
        max_coord: (f64, f64),
        x: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self::new(min_coord, max_coord, x, [0, 1, 2], material)
    }
}

impl Hit for AxisAlignedRectangle {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        let z_axis = self.axis[0];
        let x_axis = self.axis[1];
        let y_axis = self.axis[2];

        // for a ray P(t) = A + t b,
        // where A is the origin and b is the direction,
        // the intersection with the plane z = k is
        let t = (self.z - ray.origin()[z_axis]) / ray.direction()[z_axis];

        if t < t_min || t > t_max || t.is_nan() {
            return None;
        }

        // find x and y of the intersection point
        let point = ray.at(t);
        let x = point[x_axis];
        let y = point[y_axis];
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

        // the outward normal is always a unit vector along the plane's normal
        let mut normal_outward = Vec3::zeros();
        normal_outward[z_axis] = 1.0;

        Some(OutwardHitRecord::new(
            point,
            &ray,
            normal_outward,
            t,
            self.material.clone(),
            (u, v),
        ))
    }

    fn bounding_box(&self, _time_from: f64, _time_too: f64) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z
        // dimension a small amount.
        let padding = f64::EPSILON;
        let mut min = Vec3::zeros();
        let mut max = Vec3::zeros();

        min[self.axis[0]] = self.z - padding;
        max[self.axis[0]] = self.z + padding;

        min[self.axis[1]] = self.x1;
        max[self.axis[1]] = self.x1;

        min[self.axis[2]] = self.y1;
        max[self.axis[2]] = self.y1;

        Some(AABB::new(min, max))
    }
}
