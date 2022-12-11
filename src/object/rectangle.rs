use crate::Ray;
use std::sync::Arc;

use crate::{
    hit::{OutwardHitRecord, AABB},
    Hit, Material, Vec3,
};

macro_rules! axis_aligned_rectangles {
    ($x:ident, $y:ident, $z:ident) => {
        axis_aligned_rectangle!($x, $y, $z);
        axis_aligned_rectangle!($x, $z, $y);
        axis_aligned_rectangle!($y, $z, $x);

        paste! {
            axis_aligned_rectangles!(@impl enum [< $x:upper $y:upper >], [< $x:upper $z:upper >], [< $y:upper $z:upper >]);
        }
    };

    (@impl enum $($plane:ident),+) => {
        paste! {
            #[derive(Debug, Clone)]
            pub enum AxisAlignedRectangle {
                $(
                    [< $plane >]([< $plane Rectangle >]),
                )+
            }
        }

        impl Hit for AxisAlignedRectangle {
            fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
                match self {
                    $(
                        AxisAlignedRectangle::$plane(rect) => rect.hit(ray, t_min, t_max),
                    )+
                }
            }

            fn bounding_box(&self, time_from: f64, time_too: f64) -> Option<AABB> {
                match self {
                    $(
                        AxisAlignedRectangle::$plane(rect) => rect.bounding_box(time_from, time_too),
                    )+
                }
            }
        }

        paste! {
            $(
                impl From<[< $plane Rectangle >]> for AxisAlignedRectangle {
                    fn from(rect: [< $plane Rectangle >]) -> Self {
                        AxisAlignedRectangle::$plane(rect)
                    }
                }
            )+
        }
    }
}

macro_rules! axis_aligned_rectangle {
    ($axis1:ident, $axis2:ident, $axis_plane:ident) => {
        axis_aligned_rectangle!(@impl $axis1, $axis2, $axis_plane);
    };
    (@impl $axis1:ident, $axis2:ident, $axis_plane:ident) => {
        paste! {
            axis_aligned_rectangle!(@impl
                $axis1,
                $axis2,
                $axis_plane,
                [<$axis1:upper $axis2:upper Rectangle>],
                [<$axis1 0>],
                [<$axis1 1>],
                [<$axis2 0>],
                [<$axis2 1>]
            );
        }
    };
    (@impl $x:ident, $y:ident, $z:ident, $sf:ident, $x0:ident, $x1:ident, $y0:ident, $y1:ident) => {
        #[derive(Debug, Clone)]
        pub struct $sf {
            $x0: f64,
            $x1: f64,
            $y0: f64,
            $y1: f64,
            $z: f64,
            material: Arc<dyn Material>,
        }

        impl $sf {
            pub fn new(
                ($x0, $y0): (f64, f64),
                ($x1, $y1): (f64, f64),
                $z: f64,
                material: Arc<dyn Material>,
            ) -> Self {
                assert!($x0 < $x1, "{} must be less than {}", stringify!($x0), stringify!($x1));
                assert!($y0 < $y1, "{} must be less than {}", stringify!($y0), stringify!($y1));
                Self {
                    $x0,
                    $x1,
                    $y0,
                    $y1,
                    $z,
                    material,
                }
            }
        }

        impl Hit for $sf {
            fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
                // for a ray P(t) = A + t b,
                // where A is the origin and b is the direction,
                // the intersection with the plane z = k is
                let t = (self.$z - ray.origin().$z()) / ray.direction().$z();
                if t < t_min || t > t_max {
                    return None;
                }

                // find x and y of the intersection point
                let point = ray.at(t);
                let $x = point.$x();
                let $y = point.$y();
                // check if the intersection point is inside the rectangle
                if $x < self.$x0 || $x > self.$x1 {
                    return None;
                }
                if $y < self.$y0 || $y > self.$y1 {
                    return None;
                }

                // find surface coordinates
                let u = ($x - self.$x0) / (self.$x1 - self.$x0);
                let v = ($y - self.$y0) / (self.$y1 - self.$y0);

                // the outward normal is always a unit vector along the plane's normal
                let normal_outward = paste! { Vec3::[<unit_ $z>]() };

                Some(
                    OutwardHitRecord::new(point, &ray, normal_outward, t, self.material.clone(), (u, v))
                        .into_against_ray(),
                )
            }

            fn bounding_box(&self, _time_from: f64, _time_too: f64) -> Option<AABB> {
                // The bounding box must have non-zero width in each dimension, so pad the Z
                // dimension a small amount.
                let padding = f64::EPSILON;
                Some(AABB::new(
                    Vec3::new(self.$x0, self.$x1, self.$z - padding),
                    Vec3::new(self.$y0, self.$y1, self.$z + padding),
                ))
            }
        }
    }
}

axis_aligned_rectangles!(x, y, z);
