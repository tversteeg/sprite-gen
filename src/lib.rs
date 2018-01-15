#![crate_name = "sprite_gen"]

extern crate rand;

pub struct Mask {
    pub size: (i32, i32),
    
    pub data: Vec<u8>,
    pub mirror_x: bool,
    pub mirror_y: bool
}

#[derive(Copy, Clone)]
pub struct Options {
    pub colored: bool,
    pub edge_brightness: f64,
    pub color_variations: f64,
    pub brightness_noise: f64,
    pub saturation: f64
}

impl Default for Options {
    pub fn default() -> Self {
        Options {
            colored: true,
            edge_brightness: 0.3,
            color_variations: 0.2,
            brightness_noise: 0.3,
            saturation: 0.5
        }
    }
}

pub fn gen_sprite(mask: &Mask, options: Options) -> ((i32, i32), Vec<u32>) {
    let size = mask.size;
    let mut dst = vec![0; (size.0 * size.1) as usize];

    (size, dst)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
