use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
    },
};

use rand::Rng;

pub const COLOR_MAX: f64 = 255.0;
pub const EPSILON: f64 = 1.0e-8;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec3<T: Copy>([T; 3]);

pub type Point3 = Vec3<f64>;
pub type Color = Vec3<f64>;

impl<T: Copy> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }

    pub fn from_array(arr: [T; 3]) -> Self {
        Self(arr)
    }

    pub fn constant(value: T) -> Self
    where
        T: Copy + Default,
    {
        Self([value, value, value])
    }

    pub fn x(&self) -> T {
        self[0]
    }

    pub fn y(&self) -> T {
        self[1]
    }

    pub fn z(&self) -> T {
        self[2]
    }

    pub fn r(&self) -> T {
        self[0]
    }

    pub fn g(&self) -> T {
        self[1]
    }

    pub fn b(&self) -> T {
        self[2]
    }

    pub fn into_array(self) -> [T; 3] {
        self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn apply<F: Fn(T) -> T>(&self, f: F) -> Self {
        Self::new(f(self.x()), f(self.y()), f(self.z()))
    }

    pub fn apply_binary<F: Fn(T, T) -> T>(&self, other: &Self, f: F) -> Self {
        Self::new(
            f(self.x(), other.x()),
            f(self.y(), other.y()),
            f(self.z(), other.z()),
        )
    }

    pub fn apply_mut<F: Fn(&mut T)>(mut self, f: F) {
        f(&mut self[0]);
        f(&mut self[1]);
        f(&mut self[2]);
    }

    pub fn apply_binary_mut<F: Fn(&mut T, &T)>(&mut self, other: &Self, f: F) {
        f(&mut self[0], &other[0]);
        f(&mut self[1], &other[1]);
        f(&mut self[2], &other[2]);
    }
}

impl<T: Copy> IntoIterator for Vec3<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T: Copy + Default> Vec3<T> {
    pub fn zero() -> Self {
        Self([T::default(); 3])
    }
}

impl<T> Vec3<T>
where
    T: Copy + Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    /// Computes the cross product of two vectors. The cross product is
    /// perpendicular to both vectors with a magnitude equal to the area of a
    /// parallelogram with the two vectors as sides.
    ///
    /// Cross product is given by the right-hand rule and is anti-commutative,
    /// i.e. `a x b = -b x a`.
    ///
    /// ![Cross product direction](https://upload.wikimedia.org/wikipedia/commons/6/6e/Cross_product.gif)
    ///
    /// This function implements the cross product as defined by the right-hand
    /// rule.
    ///
    /// # Examples
    ///
    /// ```
    /// use ray_tracing_in_one_weekend::Vec3;
    /// let a = Vec3::new(1.0, 0.0, 0.0);
    /// let b = Vec3::new(0.0, 1.0, 0.0);
    /// let c = Vec3::new(0.0, 0.0, 1.0);
    /// assert_eq!(a.cross(b), c);
    /// assert_eq!(b.cross(c), a);
    /// assert_eq!(c.cross(a), b);
    /// ```
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    /// Computes the dot product of two vectors.
    ///
    /// The dot product is computed as a[0] * b[0] + a[1] * b[1] + a[2] * b[2].
    ///
    /// # Examples
    ///
    /// ```
    /// use ray_tracing_in_one_weekend::Vec3;
    /// let x = Vec3::new(1.0, 2.0, 3.0);
    /// let y = Vec3::new(4.0, 5.0, 6.0);
    /// let dot = x.dot(y);
    /// assert_eq!(dot, 32.0);
    /// ```
    pub fn dot(&self, other: Self) -> T {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn len_squared(self) -> T {
        self.dot(self)
    }
}

impl Vec3<f64> {
    /// Generate a random vector with components in the `range`.
    pub fn random(range: Range<f64>) -> Self {
        let mut rng = rand::thread_rng();

        Self([
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        ])
    }

    pub fn unit() -> Self {
        Self([1.0, 1.0, 1.0])
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        assert!(!self.is_near_zero());
        self / self.len()
    }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        self.apply(|x| x.clamp(min, max))
    }

    pub fn abs(&self) -> Self {
        self.apply(|x| x.abs())
    }

    pub fn round(&self) -> Self {
        self.apply(|x| x.round())
    }

    pub fn sqrt(&self) -> Self {
        self.apply(|x| x.sqrt())
    }

    pub fn is_near_zero(self) -> bool {
        self.abs().0.iter().all(|&x| x < EPSILON)
    }

    pub fn max(&self, other: &Self) -> Self {
        self.apply_binary(other, |x, y| x.max(y))
    }

    pub fn min(&self, other: &Self) -> Self {
        self.apply_binary(other, |x, y| x.min(y))
    }

    pub fn max_component(&self) -> f64 {
        self.0.iter().fold(f64::NEG_INFINITY, |acc, &x| acc.max(x))
    }

    pub fn min_component(&self) -> f64 {
        self.0.iter().fold(f64::INFINITY, |acc, &x| acc.min(x))
    }

    /// Reflects the vector about a `normal`.
    /// The reflect vector is v + 2 b, where b is the projection of v onto
    /// the normal.
    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }

    /// Refracts the vector through a `normal` with a given `refraction_ratio`.
    /// The refracted vector is computed using Snell's law.
    ///
    /// The refraction ratio, or eta ratio,
    /// is the ratio of the indices of refraction of the two
    /// media. For example, if the vector is in air and the normal is in glass,
    /// the eta ratio is 1.0 / 1.5.
    pub fn refract(self, normal: Self, refraction_ratio: f64) -> Self {
        // assume that the vector and normal are normalized
        assert!(self.is_near_zero() || (self.len() - 1.0).abs() < EPSILON);
        assert!(normal.is_near_zero() || (normal.len() - 1.0).abs() < EPSILON);
        // cos(theta) is the dot product of the vector and the normal
        let cos_theta = (-self).dot(normal).min(1.0);
        let r_out_perpendicular = refraction_ratio * (self + cos_theta * normal);
        let r_out_parallel = (1.0 - r_out_perpendicular.len_squared()).abs().sqrt().neg() * normal;
        let r_out = r_out_perpendicular + r_out_parallel;
        assert!(r_out_parallel.dot(r_out_perpendicular).abs() < EPSILON);
        r_out
    }

    /// Linearly interpolate between two vectors.
    /// The interpolation parameter `t` should be in the range [0, 1].
    /// 
    /// # Parameters
    /// 
    /// * `t` - The interpolation parameter.
    /// * `other` - The other vector to interpolate with.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ray_tracing_in_one_weekend::Vec3;
    /// let a = Vec3::new(1.0, 2.0, 3.0);
    /// let b = Vec3::new(4.0, 5.0, 6.0);
    /// let c = Vec3::new(2.5, 3.5, 4.5);
    /// assert_eq!(a.lerp(b, 0.5), c);
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if `t` is not in the range [0, 1].
    /// ```should_panic
    /// use ray_tracing_in_one_weekend::Vec3;
    /// let a = Vec3::new(1.0, 2.0, 3.0);
    /// let b = Vec3::new(4.0, 5.0, 6.0);
    /// a.lerp(b, 1.5);
    /// ```
    pub fn lerp(self, other: Self, t: f64) -> Self {
        (1.0 - t) * self + t * other
    }
}

impl<T: Copy> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T: Copy> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

macro_rules! vec3_operator {
    ($trait:ident, $fn:ident, $op:tt) => {
        vec3_operator!($trait, $fn, $op, scalar);
        vec3_operator!($trait, $fn, $op, vector);
    };

    ($trait:ident, $fn:ident, $op:tt, scalar) => {
        impl $trait<Vec3<f64>> for f64 {
            type Output = Vec3<f64>;

            fn $fn(self, other: Vec3<f64>) -> Self::Output {
                other $op self
            }
        }

        impl $trait<&Vec3<f64>> for f64 {
            type Output = Vec3<f64>;

            fn $fn(self, other: &Vec3<f64>) -> Self::Output {
                Vec3([self $op other[0], self $op other[1], self $op other[2]])
            }
        }

        impl<T: $trait<Output = T> + Copy> $trait<T> for Vec3<T> {
            type Output = Self;

            fn $fn(self, other: T) -> Self::Output {
                Self([self[0] $op other, self[1] $op other, self[2] $op other])
            }
        }

        impl<T: $trait<Output = T> + Copy> $trait<T> for &Vec3<T> {
            type Output = Vec3<T>;

            fn $fn(self, other: T) -> Self::Output {
                Vec3([self[0] $op other, self[1] $op other, self[2] $op other])
            }
        }
    };

    ($trait:ident, $fn:ident, $op:tt, vector) => {
        impl<T: $trait<Output = T> + Copy> $trait for Vec3<T> {
            type Output = Self;

            fn $fn(self, other: Self) -> Self::Output {
                Self([self[0] $op other[0], self[1] $op other[1], self[2] $op other[2]])
            }
        }

        impl<T: $trait<Output = T> + Copy> $trait for &Vec3<T> {
            type Output = Vec3<T>;

            fn $fn(self, other: Self) -> Self::Output {
                Vec3([self[0] $op other[0], self[1] $op other[1], self[2] $op other[2]])
            }
        }
    };
}

macro_rules! vec3_unary_operator {
    ($trait:ident, $fn:ident, $op:tt) => {
        vec3_unary_operator!($trait, $fn, $op, vector);
        vec3_unary_operator!($trait, $fn, $op, scalar);
    };

    ($trait:ident, $fn:ident, $op:tt, vector) => {
        impl<T: Copy + $trait> $trait for Vec3<T> {
            fn $fn(&mut self, other: Self) {
                self[0] $op other[0];
                self[1] $op other[1];
                self[2] $op other[2];
            }
        }

        impl<T: Copy + $trait> $trait for &mut Vec3<T> {
            fn $fn(&mut self, other: Self) {
                self[0] $op other[0];
                self[1] $op other[1];
                self[2] $op other[2];
            }
        }
    };

    ($trait:ident, $fn:ident, $op:tt, scalar) => {
        impl<T: Copy + $trait> $trait<T> for Vec3<T> {
            fn $fn(&mut self, other: T) {
                self[0] $op other;
                self[1] $op other;
                self[2] $op other;
            }
        }

        impl<T: Copy + $trait<T>> $trait<T> for &mut Vec3<T> {
            fn $fn(&mut self, other: T) {
                self[0] $op other;
                self[1] $op other;
                self[2] $op other;
            }
        }
    };
}

vec3_unary_operator!(AddAssign, add_assign, +=);
vec3_unary_operator!(SubAssign, sub_assign, -=);
vec3_unary_operator!(MulAssign, mul_assign, *=);
vec3_unary_operator!(DivAssign, div_assign, /=);

vec3_operator!(Add, add, +);
vec3_operator!(Sub, sub, -);
vec3_operator!(Mul, mul, *);
vec3_operator!(Div, div, /);

impl<T: Copy> Neg for Vec3<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.apply(|x| -x)
    }
}

impl<T: Copy + Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        write!(
            f,
            "({:.*}, {:.*}, {:.*})",
            precision, self[0], precision, self[1], precision, self[2]
        )
    }
}

impl Color {
    /// For a given color, return the PPM color string.
    /// The color is clamped to [0, 255], and then rounded to the nearest integer.
    ///
    /// # Note
    /// - The PPM color string is of the form "R G B".
    /// - Colors are "gamma corrected" by raising them to the power of 1/2.
    pub fn format_color(&self) -> String {
        let color = self;
        let color = color.sqrt().clamp(0.0, 0.999);
        let color = (COLOR_MAX * color).round();

        format!(
            "{} {} {}",
            color[0] as u64, color[1] as u64, color[2] as u64
        )
    }

    pub fn white() -> Self {
        Self::unit()
    }

    pub fn black() -> Self {
        Self::zero()
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
}

impl Point3 {
    /// Generate a random point in a unit radius sphere centered at the origin.
    ///
    /// The generation uses the rejection method.
    /// First pick a random point in a unit cube, then reject it if
    /// it is outside the unit sphere.
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.len() < 1.0 {
                return v;
            }
        }
    }

    /// Generate a random point inside unit hemisphere of the given normal,
    /// centered at the origin.
    pub fn random_in_unit_hemisphere(normal: Vec3<f64>) -> Point3 {
        let v = Self::random_in_unit_sphere();
        if v.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            v
        } else {
            -v
        }
    }

    /// Generate a random point inside unit disk on the XY plane,
    /// centered at the origin.
    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();

        loop {
            let v = Self::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if v.len() < 1.0 {
                return v;
            }
        }
    }

    /// Generate a random point in a disk of `radius` centered at the origin.
    pub fn random_in_disk(radius: f64) -> Self {
        if radius <= EPSILON {
            return Self::zero();
        }

        let mut rng = rand::thread_rng();
        let range = -radius..radius;

        loop {
            let v = Self::new(
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone()),
                0.0,
            );
            if v.len() < 1.0 {
                return v * radius;
            }
        }
    }
}
