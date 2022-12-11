use crate::{Point3, Ray, Vec3};

/// Axis aligned bounding box (AABB).
/// AABBs are used to determine whether two objects are colliding.
///
/// For example, suppose you create a AABB of 10 objects. Any ray that misses the
/// bounding box definitely misses all 10 objects. If the ray hits the bounding box,
/// then it might be hitting one of the 10 objects. And all we need to know is whether
/// or not we hit it; we don’t need hit points or normals or any of that stuff that we
/// need for an object we want to display.
#[derive(Debug, Clone)]
pub struct AABB {
    /// Minimum point of the AABB
    pub min: Point3,
    /// Maximum point of the AABB
    pub max: Point3,
}

impl AABB {
    pub const EMPTY: Self = Self {
        min: Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        max: Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
    };

    pub fn new(min: Point3, max: Point3) -> Self {
        assert!(min.x() <= max.x());
        assert!(min.y() <= max.y());
        assert!(min.z() <= max.z());
        Self { min, max }
    }

    pub fn is_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        // t_min and t_max are the intersection points of the ray with the AABB
        let mut t_min = t_min;
        let mut t_max = t_max;

        // iterate over all three axes
        // when t_min is greater than t_max, the ray misses the AABB
        for i in 0..self.min.len() {
            let origin = ray.origin()[i];
            let direction = ray.direction()[i];
            let min = self.min[i];
            let max = self.max[i];

            // t0 and t1 are the intersection points of the ray with the slab
            // line: point = origin + t * direction
            let t0 = (min - origin) / direction;
            let t1 = (max - origin) / direction;

            // t0 and t1 are swapped if the ray is pointing in the opposite direction
            let (t0, t1) = if direction < 0.0 { (t1, t0) } else { (t0, t1) };
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            // if t_max < t_min, then the slab is missed
            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    /// Combines two AABBs into a single AABB that contains both.
    pub fn merge(&self, other: &Self) -> Self {
        let min = self.min.min(&other.min);
        let max = self.max.max(&other.max);

        Self::new(min, max)
    }

    /// Includes a point in the AABB.
    pub fn include(self, point: &Point3) -> Self {
        let min = self.min.min(point);
        let max = self.max.max(point);

        Self::new(min, max)
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    pub fn move_by(self, offset: Vec3<f64>) -> AABB {
        Self { min: self.min + offset, max: self.max + offset }
    }

    /// Returns the point at the corner of the AABB with the given index.
    /// The index is a bit mask where the first bit is the x coordinate, the second
    /// bit is the y coordinate, and the third bit is the z coordinate.
    /// The bit is 0 if the coordinate is the minimum coordinate, and 1 if it is the
    /// maximum coordinate.
    pub fn corner(&self, index: usize) -> Point3 {
        assert!(index < (1 << 3));
        let x = if index & (1 << 0) == 0 { self.min.x() } else { self.max.x() };
        let y = if index & (1 << 1) == 0 { self.min.y() } else { self.max.y() };
        let z = if index & (1 << 2) == 0 { self.min.z() } else { self.max.z() };
        Point3::new(x, y, z)
    }

    pub fn iter_corners(&self) -> impl Iterator<Item = Point3> + '_ {
        (0..(1 << 3)).map(move |i| self.corner(i))
    }

    pub fn into_iter_corners(self) -> impl Iterator<Item = Point3> {
        (0..(1 << 3)).map(move |i| self.corner(i))
    }
}
