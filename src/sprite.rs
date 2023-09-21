use std::{borrow::Cow, f64::consts::TAU, num::NonZeroU16};

use assets_manager::{
    loader::{Loader, TomlLoader},
    AnyCache, Asset, BoxedError, Compound, SharedString,
};
use blit::{slice::Slice, Blit, BlitBuffer, BlitOptions, ToBlitBuffer};
use image::ImageFormat;
use vek::{Extent2, Vec2};

use crate::SIZE;

/// Sprite that can be drawn on the  canvas.
#[derive(Debug)]
pub struct Sprite {
    /// Pixels to render.
    sprite: BlitBuffer,
}

impl Sprite {
    /// Create a sprite from a buffer of colors.
    pub fn from_buffer(buffer: &[u32], size: Extent2<usize>) -> Self {
        let sprite = BlitBuffer::from_buffer(buffer, size.w, 127);

        Self { sprite }
    }

    /// Draw the sprite.
    pub fn render(&self, canvas: &mut [u32], offset: Vec2<f64>) {
        self.sprite.blit(
            canvas,
            SIZE.into_tuple().into(),
            &BlitOptions::new_position(offset.x as i32, offset.y as i32),
        );
    }

    /// Draw the sprite as a slice9 scaling.
    pub fn render_vertical_slice(
        &self,
        canvas: &mut [u32],
        offset: Vec2<f64>,
        width: f64,
        slice: Slice,
    ) {
        self.sprite.blit(
            canvas,
            SIZE.into_tuple().into(),
            &BlitOptions::new_position(offset.x as i32, offset.y as i32)
                .with_vertical_slice(slice)
                .with_area((width, self.height())),
        );
    }

    /// Draw the sprite as a slice9 scaling.
    pub fn render_options(&self, canvas: &mut [u32], blit_options: &BlitOptions) {
        self.sprite
            .blit(canvas, SIZE.into_tuple().into(), blit_options);
    }

    /// Whether a pixel on the image is transparent.
    pub fn is_pixel_transparent(&self, pixel: Vec2<u32>) -> bool {
        let offset: Vec2<i32> = pixel.as_();

        let index: i32 = offset.x + offset.y * self.sprite.width() as i32;
        let pixel = self.sprite.pixels()[index as usize];

        pixel == 0
    }

    /// Width of the image.
    pub fn width(&self) -> u32 {
        self.sprite.width()
    }

    /// Height of the image.
    pub fn height(&self) -> u32 {
        self.sprite.height()
    }

    /// Size of the image.
    pub fn size(&self) -> Extent2<u32> {
        Extent2::new(self.width(), self.height())
    }

    /// Raw buffer.
    pub fn into_blit_buffer(self) -> BlitBuffer {
        self.sprite
    }

    /// Get the raw pixels.
    pub fn pixels_mut(&mut self) -> &mut [u32] {
        self.sprite.pixels_mut()
    }
}

impl Asset for Sprite {
    // We only support PNG images currently
    const EXTENSION: &'static str = "png";

    type Loader = SpriteLoader;
}

/// Sprite asset loader.
pub struct SpriteLoader;

impl Loader<Sprite> for SpriteLoader {
    fn load(content: Cow<[u8]>, _ext: &str) -> Result<Sprite, assets_manager::BoxedError> {
        let sprite = image::load_from_memory_with_format(&content, ImageFormat::Png)?
            .into_rgba8()
            .to_blit_buffer_with_alpha(127);

        Ok(Sprite { sprite })
    }
}
