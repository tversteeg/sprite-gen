extern crate sprite_gen;
extern crate blit;
extern crate minifb;

use sprite_gen::*;
use blit::*;
use minifb::*;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

const GRID_SQUARE_SIZE: usize = 12;

const MASK_COLOR: u32 = 0xFFFF00FF;

fn draw_grid(buffer: &mut Vec<u32>, pos: (usize, usize), size: (usize, usize)) {
    let width = size.0 * GRID_SQUARE_SIZE;
    let height = size.1 * GRID_SQUARE_SIZE;

    for y in pos.1..pos.1 + width + 1 {
        for x in pos.0..pos.0 + height + 1 {
            let index = x + y * WIDTH;

            if (y - pos.1) % GRID_SQUARE_SIZE == 0 || (x - pos.0) % GRID_SQUARE_SIZE == 0 {
                buffer[index] = 0;
            }
        }
    }
}

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

    let options = Options {
        mirror_x: true,
        ..Options::default()
    };

    for y in 0..HEIGHT / 14 {
        for x in 0..WIDTH / 14 {
            let buf = BlitBuffer::from_u32(&gen_sprite(&mask, 6, options), 12, 0xFFFFFFFF);
            buf.blit(&mut buffer, (WIDTH as i32, HEIGHT as i32), (x as i32 * 14, y as i32 * 14));
        }
    }

    //draw_grid(&mut buffer, (10, 10), (6, 12));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer).unwrap();
    }
}
