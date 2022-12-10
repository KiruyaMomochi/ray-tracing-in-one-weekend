mod color;
mod point3;

pub use color::Color;
pub use point3::Point3;

use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
    },
};

use num::{clamp, traits::FloatConst, One, Zero};
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec3<T: Copy>([T; 3]);

impl<T: Copy + Default> Default for Vec3<T> {
    fn default() -> Self {
        Self([T::default(); 3])
    }
}

impl<T: Copy> Vec3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }

    pub fn from_array(arr: [T; 3]) -> Self {
        Self(arr)
    }

    pub fn constant(value: T) -> Self
    where
        T: Copy,
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

    pub fn into_tuple(self) -> (T, T, T) {
        (self[0], self[1], self[2])
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    pub fn apply<R: Copy, F: Fn(T) -> R>(&self, f: F) -> Vec3<R> {
        Vec3::<R>::new(f(self.x()), f(self.y()), f(self.z()))
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

impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(arr: [T; 3]) -> Self {
        Self(arr)
    }
}

impl<T: Copy> From<(T, T, T)> for Vec3<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self([x, y, z])
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
    pub fn zeros() -> Self {
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
    /// use rtweekend::Vec3;
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
    /// use rtweekend::Vec3;
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

impl<T> Vec3<T>
where
    T: Copy + One,
{
    pub fn ones() -> Self {
        Self::constant(T::one())
    }
}

impl<T> Vec3<T>
where
    T: Copy + One + Zero,
{
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }

    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }

    pub fn unit_z() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }
}

impl<T> Vec3<T>
where
    T: Copy + SampleUniform,
    Range<T>: SampleRange<T>,
{
    /// Generate a random vector with components in the `range`.
    pub fn random(range: Range<T>) -> Self {
        let mut rng = rand::thread_rng();

        Self([
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        ])
    }
}

impl<T> Vec3<T>
where
    T: Copy + PartialOrd,
{
    pub fn clamp(&self, min: T, max: T) -> Self {
        self.apply(|x| clamp(x, min, max))
    }
}

// static EPSILON: T = T::epsilon() * T::from(100.0).unwrap();

pub trait Float
where
    Self: num::Float,
{
    const EPSILON: Self;
}

impl Float for f64 {
    const EPSILON: Self = 1e-10;
}

impl Float for f32 {
    const EPSILON: Self = 1e-8;
}

impl<T> Vec3<T>
where
    T: Copy + Float,
{
    /// The l2 norm of the vector.
    /// It is the square root of the sum of the squares of the components.
    /// It also equals the square root of the dot product of the vector with itself.
    ///
    /// # Examples
    /// ```
    /// use rtweekend::Vec3;
    /// let x = Vec3::new(3.0, 4.0, 0.0);
    /// assert_eq!(x.len(), 5.0);
    /// ```
    pub fn norm(&self) -> T {
        self.len_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        assert!(!self.is_near_zero());
        self / self.norm()
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
        self.abs().0.iter().all(|&x| x < T::EPSILON)
    }

    pub fn max(&self, other: &Self) -> Self {
        self.apply_binary(other, |x, y| x.max(y))
    }

    pub fn min(&self, other: &Self) -> Self {
        self.apply_binary(other, |x, y| x.min(y))
    }

    pub fn max_component(&self) -> T {
        self.0.iter().fold(T::neg_infinity(), |acc, &x| acc.max(x))
    }

    pub fn min_component(&self) -> T {
        self.0.iter().fold(T::infinity(), |acc, &x| acc.min(x))
    }

    /// Reflects the vector about a `normal`.
    /// The reflect vector is v + 2 b, where b is the projection of v onto
    /// the normal.
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * self.dot(normal) * T::from(2.0).unwrap()
    }

    /// Refracts the vector through a `normal` with a given `refraction_ratio`.
    /// The refracted vector is computed using Snell's law.
    ///
    /// The refraction ratio, or eta ratio,
    /// is the ratio of the indices of refraction of the two
    /// media. For example, if the vector is in air and the normal is in glass,
    /// the eta ratio is 1.0 / 1.5.
    pub fn refract(self, normal: Self, refraction_ratio: T) -> Self {
        // assume that the vector and normal are normalized
        assert!(self.is_near_zero() || (self.norm() - T::one()).abs() < T::EPSILON);
        assert!(normal.is_near_zero() || (normal.norm() - T::one()).abs() < T::EPSILON);
        // cos(theta) is the dot product of the vector and the normal
        let cos_theta = (-self).dot(normal).min(T::one());
        let r_out_perpendicular = (self + normal * cos_theta) * refraction_ratio;
        let r_out_parallel = normal
            * (T::one() - r_out_perpendicular.len_squared())
                .abs()
                .sqrt()
                .neg();
        let r_out = r_out_perpendicular + r_out_parallel;
        assert!(r_out_parallel.dot(r_out_perpendicular).abs() < T::EPSILON);
        r_out
    }

    /// Linearly interpolate between two vectors.
    /// Returns `self * (1 - t) + other * t`.
    /// The interpolation parameter `t` should be in the range [0, 1].
    ///
    /// # Arguments
    ///
    /// * `t` - The interpolation parameter.
    /// * `other` - The other vector to interpolate with.
    ///
    /// # Examples
    ///
    /// ```
    /// use rtweekend::Vec3;
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
    /// use rtweekend::Vec3;
    /// let a = Vec3::new(1.0, 2.0, 3.0);
    /// let b = Vec3::new(4.0, 5.0, 6.0);
    /// a.lerp(b, 1.5);
    /// ```
    pub fn lerp(self, other: Self, t: T) -> Self {
        assert!((T::from(0.0).unwrap()..=T::from(1.0).unwrap()).contains(&t));
        self * (T::one() - t) + other * t
    }

    /// Converts the vector to rectangular coordinates.
    ///
    /// # Returns
    ///
    /// A vector with `x`, `y`, and `z` components.
    ///
    /// # Examples
    ///
    /// ```
    /// use rtweekend::Vec3;
    /// use std::f64::consts::PI;
    /// const EPSILON: f64 = 1.0e-8;
    ///
    /// let v = Vec3::new(2.0_f64.sqrt(), PI / 2.0, PI / 4.0);
    /// let (x, y, z) = v.to_rectangular().into_tuple();
    /// assert!((x - -1.0).abs() < EPSILON);
    /// assert!(y.abs() < EPSILON);
    /// assert!((z - 1.0).abs() < EPSILON);
    /// ```
    ///
    pub fn to_rectangular(&self) -> Vec3<T> {
        let (r, theta, phi) = self.into_tuple();
        let x = -r * theta.sin() * phi.cos();
        let y = -r * theta.cos();
        let z = r * theta.sin() * phi.sin();
        Vec3::new(x, y, z)
    }
}

impl<T> Vec3<T>
where
    T: Copy + Float + FloatConst,
{
    /// Converts the vector to spherical coordinates.
    ///
    /// # Returns
    ///
    /// A vector with the following components:
    /// * `r` - The radius of the vector.
    /// * `theta` - The polar angle of the vector.
    /// * `phi` - The azimuthal angle of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rtweekend::Vec3;
    /// use std::f64::consts::PI;
    ///
    /// let v = Vec3::new(-1.0, 0.0, 1.0);
    /// let (r, theta, phi) = v.to_spherical().into_tuple();
    /// assert_eq!(r, 2.0_f64.sqrt());
    /// assert_eq!(theta, PI / 2.0);
    /// assert_eq!(phi, PI / 4.0);
    /// ```
    pub fn to_spherical(&self) -> Vec3<T> {
        let r = self.norm();
        let x = self.x() / r;
        let y = self.y() / r;
        let z = self.z() / r;
        let theta = (-y).acos();
        let phi = (-z).atan2(x) + T::PI();
        Vec3::new(r, theta, phi)
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
        vec3_operator!($trait, $fn, $op, scalar, f32);
        vec3_operator!($trait, $fn, $op, scalar, f64);

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

    ($trait:ident, $fn:ident, $op:tt, scalar, $ty:ty) => {
        impl $trait<Vec3<$ty>> for $ty {
            type Output = Vec3<$ty>;

            fn $fn(self, other: Vec3<$ty>) -> Self::Output {
                other $op self
            }
        }

        impl $trait<&Vec3<$ty>> for $ty {
            type Output = Vec3<$ty>;

            fn $fn(self, other: &Vec3<$ty>) -> Self::Output {
                Vec3([self $op other[0], self $op other[1], self $op other[2]])
            }
        }
    }
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
