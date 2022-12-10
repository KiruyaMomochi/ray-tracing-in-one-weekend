use image::io::Reader;

use crate::Color;

use super::Texture;

type PixelFormat = u8;

/// Image texture
#[derive(Debug, Clone)]
pub struct Image {
    image: image::ImageBuffer<image::Rgb<PixelFormat>, Vec<PixelFormat>>,
}

impl Image {
    pub fn new(image: image::ImageBuffer<image::Rgb<PixelFormat>, Vec<PixelFormat>>) -> Self {
        Self { image }
    }
    pub fn open(path: &str) -> Result<Self, image::ImageError> {
        let image = Reader::open(path)?.decode()?;
        Ok(Self::new(image.to_rgb8()))
    }
}

impl Texture for Image {
    fn color(&self, _point: crate::Point3, u: f64, v: f64) -> Color {
        // - epsilons are used to prevent out-of-bounds errors
        let u = u.clamp(0.0, 1.0 - f64::EPSILON);
        // Flip v to image coordinates
        let v = 1.0 - v.clamp(0.0, 1.0 - f64::EPSILON);

        let width = self.image.width() as f64;
        let height = self.image.height() as f64;

        let x = (width * u) as u32;
        let y = (height * v) as u32;

        self.image.get_pixel(x, y).0.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_image() {
        let image = Reader::open("img/earthmap.jpg")
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba32f();
        assert_eq!(image.get_pixel(0, 0).0, [1.0, 1.0, 1.0, 1.0]);
    }
}
