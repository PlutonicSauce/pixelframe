use crate::animation::Animation;
use crate::terminal::TerminalManager;
use std::io;
use std::thread;
use std::time::Duration;

/// Plays pixel art animations frame by frame in the terminal
pub struct AnimationPlayer {
    animation: Animation,
    frame_delay: u64,
}

impl AnimationPlayer {
    /// Creates a new animation player with specified frame delay in milliseconds
    pub fn new(animation: Animation, frame_delay: u64) -> Self {
        AnimationPlayer {
            animation,
            frame_delay,
        }
    }

    /// Plays the animation in a loop until interrupted
    pub fn play(&self) -> io::Result<()> {
        let mut terminal = TerminalManager::new()?

        loop {
            for frame_idx in 0..self.animation.frame_count() {
                // Render the frame
                if let Some(frame) = self.animation.get_frame(frame_idx) {
                    terminal.clear_screen()?
                    self.render_frame(&mut terminal, frame)?;
                }

                // Wait for the next frame
                thread::sleep(Duration::from_millis(self.frame_delay));
            }
        }
    }

    /// Renders a single frame to the terminal
    fn render_frame(
        &self,
        terminal: &mut TerminalManager,
        frame: &crate::frame::Frame,
    ) -> io::Result<()> {
        let width = frame.width();
        let height = frame.height();

        // Scale down for terminal display (terminal char width is roughly 2:1)
        let scale_x = (width as f32 / 40.0).max(1.0) as u32;
        let scale_y = (height as f32 / 20.0).max(1.0) as u32;

        for y in (0..height).step_by(scale_y as usize) {
            for x in (0..width).step_by(scale_x as usize) {
                if let Some((r, g, b)) = frame.get_pixel(x, y) {
                    // Convert RGB to a simple character representation
                    let char = self.color_to_char(r, g, b);
                    terminal.print_at(
                        (x / scale_x) as u16,
                        (y / scale_y) as u16,
                        &char.to_string(),
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Converts RGB color to a grayscale character
    fn color_to_char(&self, r: u8, g: u8, b: u8) -> char {
        // Calculate brightness (luminance)
        let brightness = ((r as f32 * 0.299) + (g as f32 * 0.587) + (b as f32 * 0.114)) as u8;

        // Map brightness to ASCII characters (darkest to brightest)
        match brightness {
            0..=25 => ' ',
            26..=50 => '. ',
            51..=75 => ':',
            76..=100 => '- ',
            101..=125 => '=',
            126..=150 => '+',
            151..=175 => '*',
            176..=200 => '#',
            201..=225 => '@',
            226..=255 => '█',
        }
    }
}