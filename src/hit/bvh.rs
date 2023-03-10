use std::ops::Range;

use rand::Rng;

use crate::{Hit, hit::{AABB, OutwardHitRecord}, Ray};

/// Bounding volume hierarchy (BVH) tree node.
///
/// BVH tree is a binary tree. It can respond to the query "does this ray intersect".
#[derive(Debug)]
pub struct BVH {
    /// Bounding box of the node
    bounding_box: AABB,
    /// Left child
    left: Option<Box<dyn Hit>>,
    /// Right child
    right: Option<Box<dyn Hit>>,
}

fn sort_objects_by_axis(objects: &mut [Box<dyn Hit>], axis: usize, time_from: f64, time_to: f64) {
    objects.sort_unstable_by(|lhs, rhs| {
        let lhs = lhs
            .bounding_box(time_from, time_to)
            .expect("No bounding box in BVHNode constructor")
            .min()[axis];
        let rhs = rhs
            .bounding_box(time_from, time_to)
            .expect("No bounding box in BVHNode constructor")
            .min()[axis];

        lhs.partial_cmp(&rhs).expect("NaN in BVHNode constructor")
    })
}

impl BVH {
    /// Create a new BVH tree from a list of objects
    ///
    /// # Arguments
    ///
    /// * `objects` - List of objects
    /// * `time_from` - Start time of the animation
    /// * `time_to` - End time of the animation
    ///
    /// # Panics
    ///
    /// * If the list of objects is empty
    /// * If any object does not have a bounding box
    /// * If any bounding box has a NaN component
    pub fn new(mut objects: Vec<Box<dyn Hit>>, time_range: Range<f64>) -> Self {
        let time_from = time_range.start;
        let time_to = time_range.end;

        match objects.len() {
            0 => panic!("No objects in BVHNode constructor"),
            1 => Self {
                bounding_box: objects[0]
                    .bounding_box(time_from, time_to)
                    .expect("No bounding box in BVHNode constructor"),
                left: Some(objects.remove(0)),
                right: None,
            },
            2 => {
                let axis = rand::thread_rng().gen_range(0..3);
                sort_objects_by_axis(&mut objects, axis, time_from, time_to);

                let left = objects.remove(0);
                let right = objects.remove(0);
                let left_bounding_box = left
                    .bounding_box(time_from, time_to)
                    .expect("No bounding box in BVHNode constructor");
                let right_bounding_box = right
                    .bounding_box(time_from, time_to)
                    .expect("No bounding box in BVHNode constructor");
                let bounding_box = left_bounding_box.merge(&right_bounding_box);

                Self {
                    bounding_box,
                    left: Some(left),
                    right: Some(right),
                }
            }
            len => {
                let axis = rand::thread_rng().gen_range(0..3);
                sort_objects_by_axis(&mut objects, axis, time_from, time_to);

                // right comes first because we want to split the list in half
                let right = objects.split_off(len / 2);
                let left = objects;
                let left = Box::new(Self::new(left, time_range.clone()));
                let right = Box::new(Self::new(right, time_range));
                let bounding_box = left.bounding_box.merge(&right.bounding_box);

                Self {
                    bounding_box,
                    left: Some(left),
                    right: Some(right),
                }
            }
        }
    }
}

impl Hit for BVH {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<OutwardHitRecord> {
        if !self.bounding_box.is_hit(&ray, t_min, t_max) {
            return None;
        }

        let mut t_max = t_max;

        let left = self
            .left
            .as_ref()
            .and_then(|left| left.hit(ray.clone(), t_min, t_max));
        if let Some(left) = &left {
            t_max = t_max.min(left.t);
        }

        let right = self
            .right
            .as_ref()
            .and_then(|right| right.hit(ray, t_min, t_max));

        right.or(left)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{material::{Lambertian, Dielectric}, Point3, Sphere, texture::SolidColor};

    #[test]
    fn test_bvh_create() -> Result<(), Box<dyn std::error::Error>> {
        let objects: Vec<Box<dyn Hit>> = vec![
            Box::new(Sphere::new(
                Point3::new(0.0, 0.0, -1.0),
                0.5,
                Arc::new(Lambertian::new(SolidColor::new_rgb(0.1, 0.2, 0.5))),
            )),
            Box::new(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                Arc::new(Lambertian::new(SolidColor::new_rgb(0.8, 0.8, 0.0))),
            )),
            Box::new(Sphere::new(
                Point3::new(1.0, 0.0, -1.0),
                0.5,
                Arc::new(Dielectric::new(1.5)),
            )),
        ];

        let _ = BVH::new(objects, 0.0..1.0);
        Ok(())
    }
}
