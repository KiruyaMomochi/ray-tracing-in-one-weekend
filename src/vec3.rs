use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

const COLOR_MAX: f64 = 255.0;

#[derive(Clone, Debug, Copy)]
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

impl Vec3<f64> {
    pub fn unit() -> Self {
        Self([1.0, 1.0, 1.0])
    }
}

impl<T> Vec3<T>
where
    T: Copy + Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T>,
{
    pub fn dot(&self, other: Self) -> T {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2]
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    pub fn len_squared(self) -> T {
        self.dot(self)
    }
}

impl Vec3<f64> {
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn normalized(self) -> Self {
        self / self.len()
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
        Self::new(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
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
        Self::new(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
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

impl<T: Copy> Mul<T> for Vec3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self[0] * rhs, self[1] * rhs, self[2] * rhs)
    }
}

impl Mul<Vec3<f64>> for f64
{
    type Output = Vec3<f64>;

    fn mul(self, rhs: Vec3<f64>) -> Self::Output {
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
        Self::new(self[0] / rhs, self[1] / rhs, self[2] / rhs)
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
        Self::new(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl<T: Copy + Display> Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl Color {
    pub fn format_color(&self) -> String {
        format!(
            "{} {} {}",
            (COLOR_MAX * self[0]).round() as u64,
            (COLOR_MAX * self[1]).round() as u64,
            (COLOR_MAX * self[2]).round() as u64
        )
    }

    pub fn white() -> Self {
        Self::unit()
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
}