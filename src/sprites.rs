use blit::{prelude::Size, Blit, BlitBuffer, BlitOptions};
use sprite_gen::{MaskValue, Options};
use vek::{Extent2, Vec2};

use crate::SIZE;

/// A grid for rendering result sprites.
#[derive(Debug)]
pub struct Sprites {
    /// Top-left position of the widget in pixels.
    pub offset: Vec2<f64>,
    /// Different generated sprites.
    pub sprites: Vec<BlitBuffer>,
    /// Size of a single sprite.
    pub size: Extent2<usize>,
    /// Amount of sprites in each dimension.
    pub amount: Extent2<usize>,
}

impl Sprites {
    /// Render the sprites.
    pub fn render(&self, canvas: &mut [u32]) {
        // Draw each sprite
        for (index, sprite) in self.sprites.iter().enumerate() {
            let x =
                (index % self.amount.w) * (sprite.width() as usize + 4) + self.offset.x as usize;
            let y =
                (index / self.amount.w) * (sprite.height() as usize + 4) + self.offset.y as usize;

            sprite.blit(
                canvas,
                Size::from_tuple(SIZE.as_().into_tuple()),
                &BlitOptions::new_position(x, y),
            );
        }
    }

    /// Generate each sprite.
    pub fn generate(
        &mut self,
        mask: &[MaskValue],
        mut options: Options,
        amount: Extent2<usize>,
        scale: usize,
    ) {
        self.amount = amount;
        self.sprites = (0..(self.amount.product()))
            .map(|_| {
                // Generate sprite
                options.seed = fastrand::u64(0..u64::MAX);
                let buf = sprite_gen::gen_sprite(mask, self.size.w, options);

                let width = if options.mirror_x {
                    self.size.w * 2
                } else {
                    self.size.w
                };
                let height = if options.mirror_y {
                    self.size.h * 2
                } else {
                    self.size.h
                };

                // Buffer for the scaled pixels
                let mut scaled_buf = vec![0; buf.len() * scale * scale];
                for y in 0..height {
                    let y_index = y * width;
                    for x in 0..width {
                        let pixel = buf[x + y_index];

                        for y2 in 0..scale {
                            let y2_index = (y_index * scale + y2 * width) * scale;
                            for x2 in 0..scale {
                                scaled_buf[x * scale + x2 + y2_index] = pixel;
                            }
                        }
                    }
                }

                // Convert to blit buffer so it's easier to draw
                BlitBuffer::from_buffer(&scaled_buf, width * scale, 0)
            })
            .collect();
    }

    /// Resize the size of the canvas.
    pub fn resize(&mut self, size: Extent2<usize>) {
        self.size = size;
    }
}

impl Default for Sprites {
    fn default() -> Self {
        Self {
            offset: Vec2::zero(),
            sprites: Vec::new(),
            size: Extent2::zero(),
            amount: Extent2::zero(),
        }
    }
}
