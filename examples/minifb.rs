extern crate blit;
extern crate minifb;
extern crate sprite_gen;

use blit::*;
use minifb::*;
use sprite_gen::*;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn main() {
    let mut buffer: Vec<u32> = vec![0x00FFFFFF; WIDTH * HEIGHT];
    let options = WindowOptions {
        scale: Scale::X2,
        ..WindowOptions::default()
    };
    let mut window = Window::new("sprite-gen example - ESC to exit", WIDTH, HEIGHT, options)
        .expect("Unable to open window");

    let mask = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, -1, 0, 0, 0, 1, 1, -1, 0, 0, 0, 1, 1,
        -1, 0, 0, 1, 1, 1, -1, 0, 1, 1, 1, 2, 2, 0, 1, 1, 1, 2, 2, 0, 1, 1, 1, 2, 2, 0, 1, 1, 1, 1,
        -1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1,
    ];

    let options = Options {
        mirror_x: true,
        mirror_y: true,
        colored: false,
        ..Options::default()
    };

    let mask_size = (6, 12);
    let mut sprite_size = (mask_size.0, mask_size.1);
    if options.mirror_x {
        sprite_size.0 *= 2;
    }
    if options.mirror_y {
        sprite_size.1 *= 2;
    }
    let sprite_size_padded = (sprite_size.0 + 2, sprite_size.1 + 2);

    for y in 0..HEIGHT / sprite_size_padded.1 {
        for x in 0..WIDTH / sprite_size_padded.0 {
            // Generate the sprite and add it as a blitbuffer so it can be rendered easily on the
            // output buffer
            let buf = BlitBuffer::from_buffer(
                &gen_sprite(&mask, mask_size.0, options),
                sprite_size.0 as i32,
                Color::from_u32(0xFFFFFFFF),
            );
            let pos = (
                (x * sprite_size_padded.0) as i32,
                (y * sprite_size_padded.1) as i32,
            );
            buf.blit(&mut buffer, WIDTH, pos);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
