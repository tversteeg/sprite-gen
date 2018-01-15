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
    let mut window = Window::new("sprite-gen editor - ESC to exit", WIDTH, HEIGHT, options).expect("Unable to open window");

    let mask = [
        0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 1,
        0, 0, 0, 0, 1,-1,
        0, 0, 0, 1, 1,-1,
        0, 0, 0, 1, 1,-1,
        0, 0, 1, 1, 1,-1,
        0, 1, 1, 1, 2, 2,
        0, 1, 1, 1, 2, 2,
        0, 1, 1, 1, 2, 2,
        0, 1, 1, 1, 1,-1,
        0, 0, 0, 1, 1, 1,
        0, 0, 0, 0, 0, 0];

    for x in 0..WIDTH / 12 {
        let buf = BlitBuffer::from_u32(&gen_sprite(&mask, 6, Options::default()), 6, 0xFFFFFFFF);
        buf.blit(&mut buffer, (WIDTH as i32, HEIGHT as i32), (x as i32 * 12, 0));
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
