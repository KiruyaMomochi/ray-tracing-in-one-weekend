pub type Color = super::Vec3<f64>;

pub const COLOR_MAX: f64 = 255.0;
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

    pub fn is_valid_color(&self) -> bool {
        self.iter()
            .all(|x| x.is_finite() && (0.0..=1.0).contains(x))
    }

    pub fn white() -> Self {
        Self::ones()
    }

    pub fn black() -> Self {
        Self::zeros()
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
