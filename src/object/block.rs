use std::sync::Arc;

use crate::{Material, Point3, Hit, hit::AABB, HitRecord, Ray};
use paste::paste;

use super::rectangle::{AxisAlignedRectangle, XYRectangle, XZRectangle, YZRectangle};

/// An axis-aligned block of space.
/// It holds 6 rectangles, one for each face.
pub struct Block {
    /// The 6 rectangles that make up the block.
    rectangles: [AxisAlignedRectangle; 6],
    /// Minimum point of the block.
    min_point: Point3,
    /// Maximum point of the block.
    max_point: Point3,
}

macro_rules! rectangle {
    (($min:ident, $max:ident).($axis1:ident, $axis2:ident), $side:ident.$plane:ident, $material:expr) => {
        paste! {
            rectangle!(@impl [< $axis1:upper $axis2:upper Rectangle>], ($min.$axis1(), $min.$axis2()), ($max.$axis1(), $max.$axis2()), $side.$plane(), $material)
        }
    };
    (@impl $st:ident, $min:tt, $max:tt, $plane:expr, $material:expr) => {
        AxisAlignedRectangle::from($st::new($min, $max, $plane, $material))
    };
}

macro_rules! rectangles {
    (($min:ident, $max:ident).($x:ident, $y:ident, $z:ident), $material:ident) => {
        [
            rectangle!(($min, $max).($x, $y), $min.$z, $material.clone()),
            rectangle!(($min, $max).($x, $y), $max.$z, $material.clone()),
            rectangle!(($min, $max).($x, $z), $min.$y, $material.clone()),
            rectangle!(($min, $max).($x, $z), $max.$y, $material.clone()),
            rectangle!(($min, $max).($y, $z), $min.$x, $material.clone()),
            rectangle!(($min, $max).($y, $z), $max.$y, $material),
        ]
    };
}

impl Block {
    pub fn new(min_point: Point3, max_point: Point3, material: Arc<dyn Material>) -> Self {
        assert!(
            min_point.x() < max_point.x()
                && min_point.y() < max_point.y()
                && min_point.z() < max_point.z(),
            "Block must have positive volume"
        );

        let rectangles = rectangles!((min_point, max_point).(x, y, z), material);
        Self {
            rectangles,
            min_point,
            max_point,
        }
    }
}

impl Hit for Block {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.rectangles.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time_from: f64, _time_to: f64) -> Option<AABB> {
        Some(AABB::new(self.min_point, self.max_point))
    }
}
