pub type Color = super::Vec3<f64>;

pub const COLOR_MAX: f64 = 255.0;
impl Color {
    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0);
    pub const RED: Self = Self::new(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::new(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::new(0.0, 0.0, 1.0);

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

    pub fn is_valid_color(&self) -> bool {
        self.iter()
            .all(|x| x.is_finite() && (0.0..=1.0).contains(x))
    }

    pub fn from_rgb8(r: u8, g: u8, b: u8) -> Self {
        const COLOR_MAX: f64 = u8::MAX as f64;
        Self::new(r as f64 / COLOR_MAX, g as f64 / COLOR_MAX, b as f64 / COLOR_MAX)
    }
}

impl From<[u8; 3]> for Color {
    fn from(pixel: [u8; 3]) -> Self {
        Self::from_rgb8(pixel[0], pixel[1], pixel[2])
    }
}
