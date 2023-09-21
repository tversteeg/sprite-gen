use sprite_gen::MaskValue;
use vek::{Aabr, Extent2, Rect, Vec2};

use crate::{input::Input, SIZE};

/// A simple slider widget.
#[derive(Debug)]
pub struct Grid {
    /// Top-left position of the widget in pixels.
    pub offset: Vec2<f64>,
    /// Size of the grid.
    pub size: Vec2<u8>,
    /// Size of each grid item in pixels.
    pub scaling: Vec2<u8>,
    /// Each value of the grid.
    pub values: Vec<MaskValue>,
}

impl Grid {
    /// Handle the input.
    pub fn update(&mut self, input: &Input) {}

    /// Render the slider.
    pub fn render(&self, canvas: &mut [u32]) {
        // Fill background with white
        for y in 0..self.height() {
            let start = self.offset.x as usize + (self.offset.y as usize + y) * SIZE.w;
            let y_descaled = y / self.scaling.y as usize;

            for x in 0..self.size.x as usize {
                let start = start + x * self.scaling.x as usize;

                // Offset grid pattern
                let is_filled =
                    (y_descaled % 2 == 0 && x % 2 == 0) || (y_descaled % 2 == 1 && x % 2 == 1);

                canvas[start..(start + self.scaling.x as usize)].fill(if is_filled {
                    0xFFFFFFFF
                } else {
                    0xFFEEEEEE
                });
            }
        }
    }

    /// Resize the grid.
    pub fn resize(&mut self, size: Vec2<u8>, scaling: Vec2<u8>) {
        self.size = size;
        self.scaling = scaling;
    }

    /// Total width.
    pub fn width(&self) -> usize {
        self.size.x as usize * self.scaling.x as usize
    }

    /// Total height.
    pub fn height(&self) -> usize {
        self.size.y as usize * self.scaling.y as usize
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            offset: Vec2::zero(),
            size: Vec2::zero(),
            scaling: Vec2::one(),
            values: Vec::new(),
        }
    }
}
