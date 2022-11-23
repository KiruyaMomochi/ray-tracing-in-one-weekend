use std::{
    fmt::Display,
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign,
    },
};

use rand::Rng;

const COLOR_MAX: f64 = 255.0;
const EPSILON: f64 = 1.0e-8;

#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec3<T: Copy>([T; 3]);

pub type Point3 = Vec3<f64>;
pub type Color = Vec3<f64>;

impl<T: Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }

    pub fn from_array(arr: [T; 3]) -> Self {
        Self(arr)
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
        self / self.len()
    }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Self([
            self[0].clamp(min, max),
            self[1].clamp(min, max),
            self[2].clamp(min, max),
        ])
    }

    pub fn round(&self) -> Self {
        Self([self[0].round(), self[1].round(), self[2].round()])
    }

    pub fn sqrt(&self) -> Self {
        Self([self[0].sqrt(), self[1].sqrt(), self[2].sqrt()])
    }

    pub fn near_zero(self) -> bool {
        self.0.iter().all(|&x| x.abs() < EPSILON)
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
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

impl<T: Copy> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl<T: Copy> AddAssign for Vec3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl<T: Copy> Sub for Vec3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl<T: Copy> SubAssign for Vec3<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
    }
}

impl<T: Copy> Add<T> for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Self::Output::new(self[0] + rhs, self[1] + rhs, self[2] + rhs)
    }
}

impl Add<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn add(self, rhs: Vec3<f64>) -> Self::Output {
        Vec3::new(self + rhs[0], self + rhs[1], self + rhs[2])
    }
}

impl<T: Copy> Mul<T> for Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl<T: Copy> Mul<T> for &Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl Mul<Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, rhs: Vec3<f64>) -> Self::Output {
        Vec3::new(self * rhs[0], self * rhs[1], self * rhs[2])
    }
}

impl Mul<&Vec3<f64>> for f64 {
    type Output = Vec3<f64>;

    fn mul(self, rhs: &Vec3<f64>) -> Self::Output {
        Vec3::new(self * rhs[0], self * rhs[1], self * rhs[2])
    }
}

impl<T: Copy> MulAssign<T> for Vec3<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl<T: Copy> Div<T> for Vec3<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
    }
}

impl<T: Copy> DivAssign<T> for Vec3<T>
where
    T: DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

impl<T: Copy> Mul for Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl<T: Copy> Mul for &Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Vec3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl<T: Copy> Neg for Vec3<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output::new(-self[0], -self[1], -self[2])
    }
}

impl<T: Copy + Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        write!(
            f,
            "{:.*} {:.*} {:.*}",
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
    /// Generate a random point in a unit radius sphere
    ///
    /// The generation uses the rejection method.
    /// First pick a random point in a unit cube, then reject it if
    /// it is outside the unit sphere.
    pub fn random_in_sphere() -> Self {
        loop {
            let v = Vec3::random(-1.0..1.0);
            if v.len() < 1.0 {
                return v;
            }
        }
    }

    /// Generate a random point in the unit hemisphere
    /// of the given normal.
    pub fn random_in_hemisphere(normal: Vec3<f64>) -> Point3 {
        let v = Self::random_in_sphere();
        if v.dot(normal) > 0.0 {
            // In the same hemisphere as the normal
            v
        } else {
            -v
        }
    }
}
