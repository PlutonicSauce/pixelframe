use crate::frame::Frame;
use image::{DynamicImage, ImageReader};
use std::io;
use std::path::Path;

/// Represents a pixel art animation loaded from an image file.
/// The image is divided into frames based on its dimensions.
pub struct Animation {
    frames: Vec<Frame>,
    width: u32,
    height: u32,
}

impl Animation {
    /// Loads an image and splits it into frames.
    /// For now, treats the entire image as a single frame.
    /// Can be extended to support sprite sheets by dividing into grid.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();

        let img = ImageReader::open(path)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?
            .decode()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        let (width, height) = img.dimensions();
        let rgba_img = img.to_rgba8();

        // Create a single frame from the entire image
        let frame = Frame::from_rgba(&rgba_img, width, height);

        Ok(Animation {
            frames: vec![frame],
            width,
            height,
        })
    }

    /// Gets a specific frame by index
    pub fn get_frame(&self, index: usize) -> Option<&Frame> {
        self.frames.get(index)
    }

    /// Returns total number of frames
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Returns animation width in pixels
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns animation height in pixels
    pub fn height(&self) -> u32 {
        self.height
    }
}