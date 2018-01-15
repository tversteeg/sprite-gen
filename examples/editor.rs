extern crate sprite_gen;
extern crate blit;
extern crate minifb;

use sprite_gen::*;
use blit::*;
use minifb::*;

const WIDTH: usize = 250;
const HEIGHT: usize = 250;

const MASK_COLOR: u32 = 0xFFFF00FF;

fn main() {
    let mut buffer: Vec<u32> = vec![0x00FFFFFF; WIDTH * HEIGHT];
    let options = WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    };
    let mut window = Window::new("sprite-gen Example - ESC to exit", WIDTH, HEIGHT, options).expect("Unable to open window");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
