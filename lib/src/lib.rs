#![crate_name = "sprite_gen"]

extern crate rand;

use rand::{Rng, XorShiftRng};

#[derive(Copy, Clone)]
pub struct Options {
    pub mirror_x: bool,
    pub mirror_y: bool,
    pub colored: bool,
    pub edge_brightness: f64,
    pub color_variations: f64,
    pub brightness_noise: f64,
    pub saturation: f64
}

impl Default for Options {
    fn default() -> Self {
        Options {
            mirror_x: false,
            mirror_y: false,
            colored: true,
            edge_brightness: 0.3,
            color_variations: 0.2,
            brightness_noise: 0.3,
            saturation: 0.5
        }
    }
}

pub fn gen_sprite(mask_buffer: &[i8], mask_width: usize, options: Options) -> Vec<u32> {
    let mask_height = mask_buffer.len() / mask_width;

    // Copy the array to this vector
    let mut mask: Vec<i8> = mask_buffer.iter().cloned().collect();

    let mut rng: XorShiftRng = rand::thread_rng().gen();

    // Generate a random sample, if it's a internal body there is a 50% chance it will be empty. If it's a regular body there is a 50% chance it will turn into a border
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

    // Convert the data to pixels
    if !options.mirror_x && !options.mirror_y {
        return mask.iter().map(|&v| match v {
            -1 => 0,
            _ => 0xFFFFFFFF
        }).collect();
    } else if options.mirror_x && !options.mirror_y {
        let width = mask_width * 2;
        let mut result = vec![0; width * mask_height];

        for y in 0..mask_height {
            for x in 0..mask_width {
                let value = match mask[x + y * mask_width] {
                    -1 => 0,
                    _ => 0xFFFFFFFF
                };
                let index = x + y * width;
                result[index] = value;
                let index = (width - x - 1) + y * width;
                result[index] = value;
            }
        }

        return result;
    }
    //TODO implement mirrorring

    mask.iter().map(|&v| match v {
        -1 => 0,
        _ => 0xFFFFFFFF
    }).collect()
}
