use image::RgbaImage;

/// Represents a single frame of pixel art.
/// Stores pixel data as RGB colors (ignoring alpha for simplicity).
#[derive(Clone)]
pub struct Frame {
    pixels: Vec<(u8, u8, u8)>, // RGB tuples
    width: u32,
    height: u32,
}

impl Frame {
    /// Creates a frame from RGBA image data
    pub fn from_rgba(img: &RgbaImage, width: u32, height: u32) -> Self {
        let pixels: Vec<(u8, u8, u8)> = img
            .pixels()
            .map(|p| (p[0], p[1], p[2])) // Extract RGB, drop alpha
            .collect();

        Frame {
            pixels,
            width,
            height,
        }
    }

    /// Gets pixel color at (x, y)
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<(u8, u8, u8)> {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.pixels.get(index).copied()
        } else {
            None
        }
    }

    /// Returns frame width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns frame height
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns all pixels as a reference
    pub fn pixels(&self) -> &[(u8, u8, u8)] {
        &self.pixels
    }
}