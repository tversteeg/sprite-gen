#![crate_name = "sprite_gen"]

extern crate hsl;
extern crate rand;

use hsl::HSL;
use rand::{Rng, XorShiftRng};

/// Replacement for the `i8` datatype that can be passed to `gen_sprite`.
#[derive(Clone, Eq, PartialEq)]
pub enum MaskValue {
    /// - `-1`: This pixel will always be a border.
    Solid,
    /// - `0`: This pixel will always be empty.
    Empty,
    /// - `1`: This pixel will either be empty or filled (body).
    Body1,
    /// - `2`: This pixel will either be a border or filled (body).
    Body2,
}

impl From<MaskValue> for i8 {
    fn from(from: MaskValue) -> Self {
        match from {
            MaskValue::Solid => -1,
            MaskValue::Empty => 0,
            MaskValue::Body1 => 1,
            MaskValue::Body2 => 2,
        }
    }
}

impl From<i8> for MaskValue {
    fn from(from: i8) -> Self {
        match from {
            -1 => MaskValue::Solid,
            1 => MaskValue::Body1,
            2 => MaskValue::Body2,
            _ => MaskValue::Empty,
        }
    }
}

impl Default for MaskValue {
    fn default() -> Self {
        MaskValue::Empty
    }
}

/// The options for the `gen_sprite` function.
#[derive(Copy, Clone)]
pub struct Options {
    /// `true` if the result buffer should be mirrored along the X axis.
    pub mirror_x: bool,
    /// `true` if the result buffer should be mirrored along the Y axis.
    pub mirror_y: bool,
    /// `true` if the output should be colored. `false` if the output should be 1-bit. The
    /// fields after this field only apply if `colorer` is `true`.
    pub colored: bool,
    /// A value from `0.0` - `1.0`.
    pub edge_brightness: f64,
    /// A value from `0.0` - `1.0`.
    pub color_variations: f64,
    /// A value from `0.0` - `1.0`.
    pub brightness_noise: f64,
    /// A value from `0.0` - `1.0`.
    pub saturation: f64,
}

impl Default for Options {
    /// - `mirror_x`: `false`
    /// - `mirror_y`: `false`
    /// - `colored`: `true`
    /// - `edge_brightness`: `0.3`
    /// - `color_variations`: `0.2`
    /// - `brightness_noise`: `0.3`
    /// - `saturation`: `0.5`
    fn default() -> Self {
        Options {
            mirror_x: false,
            mirror_y: false,
            colored: true,
            edge_brightness: 0.3,
            color_variations: 0.2,
            brightness_noise: 0.3,
            saturation: 0.5,
        }
    }
}

/// Randomly generate a new sprite.
///
/// A mask buffer of `i8` values should be passed together with the width of that buffer.
/// The height is automatically calculated by dividing the size of the buffer with the width.
/// The `i8` values should be one of the following, and will generate a bitmap:
/// - `-1`: This pixel will always be a border.
/// - `0`: This pixel will always be empty.
/// - `1`: This pixel will either be empty or filled (body).
/// - `2`: This pixel will either be a border or filled (body).
///
/// ```
/// use sprite_gen::{gen_sprite, Options, MaskValue};
///
/// let mask = vec![MaskValue::Empty; 12 * 12];
/// let buffer = gen_sprite(&mask, 12, Options::default());
/// ```
pub fn gen_sprite<T>(mask_buffer: &[T], mask_width: usize, options: Options) -> Vec<u32>
where
    T: Into<i8> + Clone,
{
    let mask_height = mask_buffer.len() / mask_width;

    // Copy the array to this vector
    let mut mask: Vec<i8> = mask_buffer
        .iter()
        .map(|v| std::convert::Into::into(v.clone()))
        .collect::<_>();

    let mut rng: XorShiftRng = rand::thread_rng().gen();

    // Generate a random sample, if it's a internal body there is a 50% chance it will be empty
    // If it's a regular body there is a 50% chance it will turn into a border
    for val in mask.iter_mut() {
        if *val == 1 {
            // Either 0 or 1
            *val = rng.next_f32().round() as i8;
        } else if *val == 2 {
            // Either -1 or 1
            *val = (rng.next_f32().round() as i8) * 2 - 1;
        }
    }

    // Generate edges
    for y in 0..mask_height {
        for x in 0..mask_width {
            let index = x + y * mask_width;
            if mask[index] <= 0 {
                continue;
            }

            if y > 0 && mask[index - mask_width] == 0 {
                mask[index - mask_width] = -1;
            }
            if y < mask_height - 1 && mask[index + mask_width] == 0 {
                mask[index + mask_width] = -1;
            }
            if x > 0 && mask[index - 1] == 0 {
                mask[index - 1] = -1;
            }
            if x < mask_width - 1 && mask[index + 1] == 0 {
                mask[index + 1] = -1;
            }
        }
    }

    // Color the mask image
    let colored: Vec<u32> = if options.colored {
        color_output(&mask, (mask_width, mask_height), &options, &mut rng)
    } else {
        onebit_output(&mask)
    };

    // Check for mirroring
    if options.mirror_x && options.mirror_y {
        // Mirror both X & Y
        let width = mask_width * 2;
        let height = mask_height * 2;
        let mut result = vec![0; width * height];

        for y in 0..mask_height {
            for x in 0..mask_width {
                let index = x + y * mask_width;
                let value = colored[index];

                let index = x + y * width;
                result[index] = value;

                let index = (width - x - 1) + y * width;
                result[index] = value;

                let index = x + (height - y - 1) * width;
                result[index] = value;

                let index = (width - x - 1) + (height - y - 1) * width;
                result[index] = value;
            }
        }

        return result;
    } else if options.mirror_x {
        // Only mirror X
        let width = mask_width * 2;
        let mut result = vec![0; width * mask_height];

        for y in 0..mask_height {
            for x in 0..mask_width {
                let index = x + y * mask_width;
                let value = colored[index];

                let index = x + y * width;
                result[index] = value;

                let index = (width - x - 1) + y * width;
                result[index] = value;
            }
        }

        return result;
    } else if options.mirror_y {
        // Only mirror Y
        let height = mask_height * 2;
        let mut result = vec![0; mask_width * height];

        for y in 0..mask_height {
            for x in 0..mask_width {
                let index = x + y * mask_width;
                let value = colored[index];
                result[index] = value;

                let index = x + (height - y - 1) * mask_width;
                result[index] = value;
            }
        }

        return result;
    }

    colored
}

#[inline]
fn onebit_output(mask: &[i8]) -> Vec<u32> {
    mask.iter()
        .map(|&v| match v {
            -1 => 0,
            _ => 0xFF_FF_FF_FF,
        })
        .collect()
}

#[inline]
fn color_output(
    mask: &[i8],
    mask_size: (usize, usize),
    options: &Options,
    rng: &mut XorShiftRng,
) -> Vec<u32> {
    let mut result = vec![0xFF_FF_FF_FF; mask.len()];

    let is_vertical_gradient = rng.next_f32() > 0.5;
    let saturation = (rng.next_f64() * options.saturation).max(0.0).min(1.0);
    let mut hue = rng.next_f64();

    let variation_check = 1.0 - options.color_variations;
    let brightness_inv = 1.0 - options.brightness_noise;

    let uv_size = if is_vertical_gradient {
        (mask_size.1, mask_size.0)
    } else {
        mask_size
    };

    for u in 0..uv_size.0 {
        let is_new_color =
            ((rng.gen_range(-1.0, 1.0) + rng.gen_range(-1.0, 1.0) + rng.gen_range(-1.0, 1.0))
                / 3.0 as f64)
                .abs();

        if is_new_color > variation_check {
            hue = rng.next_f64();
        }

        for v in 0..uv_size.1 {
            let index = if is_vertical_gradient {
                v + u * mask_size.0
            } else {
                u + v * mask_size.0
            };

            let val = mask[index];
            if val == 0 {
                continue;
            }

            let u_sin = ((u as f64 / uv_size.0 as f64) * std::f64::consts::PI).sin();
            let brightness = u_sin * brightness_inv + rng.gen_range(0.0, options.brightness_noise);

            let mut rgb = HSL {
                h: hue,
                s: saturation,
                l: brightness,
            }
            .to_rgb();

            // Make the edges darker
            if val == -1 {
                rgb.0 = (rgb.0 as f64 * options.edge_brightness) as u8;
                rgb.1 = (rgb.1 as f64 * options.edge_brightness) as u8;
                rgb.2 = (rgb.2 as f64 * options.edge_brightness) as u8;
            }

            result[index] = ((rgb.0 as u32) << 16) | ((rgb.1 as u32) << 8) | (rgb.2 as u32);
        }
    }

    result
}
