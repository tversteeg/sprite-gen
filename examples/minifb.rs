extern crate sprite_gen;
extern crate blit;
extern crate minifb;

use sprite_gen::*;
use blit::*;
use minifb::*;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn main() {
    let mut buffer: Vec<u32> = vec![0x00FFFFFF; WIDTH * HEIGHT];
    let options = WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    };
    let mut window = Window::new("sprite-gen example - ESC to exit", WIDTH, HEIGHT, options).expect("Unable to open window");

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

    let options = Options {
        mirror_x: true,
        ..Options::default()
    };

    let sprite_size = (12, 12);
    let sprite_size_padded = (sprite_size.0 + 2, sprite_size.1 + 2);

    for y in 0..HEIGHT / sprite_size_padded.1 {
        for x in 0..WIDTH / sprite_size_padded.0 {
            // Generate the sprite and add it as a blitbuffer so it can be rendered easily on the
            // output buffer
            let buf = BlitBuffer::from_buffer(&gen_sprite(&mask, 6, options), sprite_size.0 as i32, Color::from_u32(0xFFFFFFFF));
            let pos = ((x * sprite_size_padded.0) as i32, (y * sprite_size_padded.1) as i32);
            buf.blit(&mut buffer, WIDTH, pos);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
