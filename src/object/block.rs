use std::sync::Arc;

use crate::{
    hit::{OutwardHitRecord, AABB},
    Hit, Material, Point3, Ray,
};
use super::rectangle::AxisAlignedRectangle;

/// An axis-aligned block of space.
/// It holds 6 rectangles, one for each face.
#[derive(Debug, Clone)]
pub struct Block {
    /// The 6 rectangles that make up the block.
    rectangles: [AxisAlignedRectangle; 6],
    /// Minimum point of the block.
    min_point: Point3,
    /// Maximum point of the block.
    max_point: Point3,
}

macro_rules! rectangle {
    (($min:ident, $max:ident), $side:ident, [$plane:literal, $axis1:literal, $axis2:literal], $material:expr) => {
        AxisAlignedRectangle::new(
            ($min[$axis1], $min[$axis2]),
            ($max[$axis1], $max[$axis2]),
            $side[$plane],
            [$plane, $axis1, $axis2],
            $material,
        )
    };
}

macro_rules! rectangles {
    (($min:ident, $max:ident), $material:ident) => {
        [
            rectangle!(($min, $max), $min, [2, 0, 1], $material.clone()),
            rectangle!(($min, $max), $max, [2, 0, 1], $material.clone()),
            rectangle!(($min, $max), $min, [1, 0, 2], $material.clone()),
            rectangle!(($min, $max), $max, [1, 0, 2], $material.clone()),
            rectangle!(($min, $max), $min, [0, 1, 2], $material.clone()),
            rectangle!(($min, $max), $max, [0, 1, 2], $material),
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

        let rectangles = rectangles!((min_point, max_point), material);
        Self {
            rectangles,
            min_point,
            max_point,
        }
    }
}

impl Hit for Block {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        self.rectangles.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time_from: f64, _time_to: f64) -> Option<AABB> {
        Some(AABB::new(self.min_point, self.max_point))
    }
}
