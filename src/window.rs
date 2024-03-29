use std::sync::Arc;

use game_loop::winit::{dpi::LogicalSize, window::WindowBuilder};
use miette::{IntoDiagnostic, Result};
use pixels::{wgpu::BlendState, PixelsBuilder, SurfaceTexture};
use vek::{Extent2, Vec2};
use winit::{
    event::{
        ElementState, Event, KeyboardInput, MouseButton, TouchPhase, VirtualKeyCode, WindowEvent,
    },
    event_loop::EventLoop,
};

use crate::input::Input;

/// Create a new window with an event loop and run the application.
pub async fn run<G, U, R>(
    game_state: G,
    size: Extent2<usize>,
    updates_per_second: u32,
    mut update: U,
    mut render: R,
) -> Result<()>
where
    G: 'static,
    U: FnMut(&mut G, &Input) + 'static,
    R: FnMut(&mut G, &mut [u32]) + 'static,
{
    #[cfg(target_arch = "wasm32")]
    let canvas = wasm::setup_canvas();

    // Build the window builder with the event loop the user supplied
    let event_loop = EventLoop::new();
    let logical_size = LogicalSize::new(size.w as f64, size.h as f64);
    #[allow(unused_mut)]
    let mut window_builder = WindowBuilder::new()
        .with_title("Sprite")
        .with_inner_size(logical_size)
        .with_min_inner_size(logical_size);

    // Setup the WASM canvas if running on the browser
    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowBuilderExtWebSys;

        window_builder = window_builder.with_canvas(Some(canvas));
    }

    let window = window_builder.build(&event_loop).into_diagnostic()?;

    let pixels = {
        let surface_texture = SurfaceTexture::new(size.w as u32 * 2, size.h as u32 * 2, &window);
        PixelsBuilder::new(size.w as u32, size.h as u32, surface_texture)
            .clear_color(pixels::wgpu::Color::WHITE)
            .blend_state(BlendState::REPLACE)
            .build_async()
            .await
    }
    .into_diagnostic()?;

    #[cfg(target_arch = "wasm32")]
    wasm::update_canvas(size);

    // Open the window and run the event loop
    let mut buffer = vec![0u32; size.w * size.h];

    game_loop::game_loop(
        event_loop,
        Arc::new(window),
        (game_state, pixels, Input::default()),
        updates_per_second,
        0.1,
        move |g| {
            update(&mut g.game.0, &g.game.2);

            g.game.2.update();
        },
        move |g| {
            render(&mut g.game.0, &mut buffer);

            {
                // Blit draws the pixels in RGBA format, but the pixels crate expects BGRA, so convert it
                g.game
                    .1
                    .frame_mut()
                    .chunks_exact_mut(4)
                    .zip(buffer.iter())
                    .for_each(|(target, source)| {
                        let source = source.to_ne_bytes();
                        target[0] = source[2];
                        target[1] = source[1];
                        target[2] = source[0];
                        target[3] = source[3];
                    });
            }

            // Render the pixel buffer
            if let Err(err) = g.game.1.render() {
                dbg!(err);
                // TODO: properly handle error
                g.exit();
            }
        },
        move |g, ev| {
            match ev {
                // Handle close event
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => g.exit(),

                // Resize the window
                Event::WindowEvent {
                    event: WindowEvent::Resized(new_size),
                    ..
                } => {
                    g.game
                        .1
                        .resize_surface(new_size.width, new_size.height)
                        .into_diagnostic()
                        .unwrap();
                }

                // Handle key presses
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode,
                                    state,
                                    ..
                                },
                            ..
                        },
                    ..
                } => match virtual_keycode {
                    Some(VirtualKeyCode::Up | VirtualKeyCode::W) => {
                        g.game.2.up.handle_bool(state == &ElementState::Pressed)
                    }
                    Some(VirtualKeyCode::Down | VirtualKeyCode::S) => {
                        g.game.2.down.handle_bool(state == &ElementState::Pressed)
                    }
                    Some(VirtualKeyCode::Left | VirtualKeyCode::A) => {
                        g.game.2.left.handle_bool(state == &ElementState::Pressed)
                    }
                    Some(VirtualKeyCode::Right | VirtualKeyCode::D) => {
                        g.game.2.right.handle_bool(state == &ElementState::Pressed)
                    }
                    Some(VirtualKeyCode::Space) => {
                        g.game.2.space.handle_bool(state == &ElementState::Pressed)
                    }
                    Some(VirtualKeyCode::R) => {
                        g.game.2.r.handle_bool(state == &ElementState::Pressed)
                    }
                    Some(VirtualKeyCode::G) => {
                        g.game.2.g.handle_bool(state == &ElementState::Pressed)
                    }
                    Some(VirtualKeyCode::C) => {
                        g.game.2.c.handle_bool(state == &ElementState::Pressed)
                    }
                    Some(VirtualKeyCode::O) => {
                        g.game.2.o.handle_bool(state == &ElementState::Pressed)
                    }
                    Some(VirtualKeyCode::N) => {
                        g.game.2.n.handle_bool(state == &ElementState::Pressed)
                    }
                    Some(VirtualKeyCode::X) => {
                        g.game.2.x.handle_bool(state == &ElementState::Pressed)
                    }
                    // Close the window when the <ESC> key is pressed
                    Some(VirtualKeyCode::Escape) => g.exit(),
                    _ => (),
                },

                // Handle mouse pressed
                Event::WindowEvent {
                    event: WindowEvent::MouseInput { button, state, .. },
                    ..
                } => {
                    if *button == MouseButton::Left {
                        g.game
                            .2
                            .left_mouse
                            .handle_bool(*state == ElementState::Pressed);
                    } else if *button == MouseButton::Right {
                        g.game
                            .2
                            .right_mouse
                            .handle_bool(*state == ElementState::Pressed);
                    }
                }

                Event::WindowEvent {
                    event: WindowEvent::Touch(touch),
                    ..
                } => match touch.phase {
                    TouchPhase::Started => g.game.2.left_mouse.handle_bool(true),
                    TouchPhase::Moved => (),
                    TouchPhase::Ended | TouchPhase::Cancelled => {
                        g.game.2.left_mouse.handle_bool(false)
                    }
                },

                // Handle mouse move
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    // Map raw window pixel to actual pixel
                    g.game.2.mouse_pos = g
                        .game
                        .1
                        .window_pos_to_pixel((position.x as f32, position.y as f32))
                        .map(|(x, y)| Vec2::new(x as i32, y as i32))
                        // We also map the mouse when it's outside of the bounds
                        .unwrap_or_else(|(x, y)| Vec2::new(x as i32, y as i32))
                }
                _ => (),
            }
        },
    );
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use vek::Extent2;
    use wasm_bindgen::JsCast;
    use web_sys::HtmlCanvasElement;

    /// Attach the winit window to a canvas.
    pub fn setup_canvas() -> HtmlCanvasElement {
        log::debug!("Binding window to HTML canvas");

        let window = web_sys::window().unwrap();

        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.style().set_css_text("text-align: center");

        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        canvas.set_id("canvas");
        body.append_child(&canvas).unwrap();

        let header = document.create_element("h2").unwrap();
        header.set_text_content(Some("Sprite"));
        body.append_child(&header).unwrap();

        canvas
    }

    /// Update the size of the canvas.
    pub fn update_canvas(size: Extent2<usize>) {
        let window = web_sys::window().unwrap();

        let document = window.document().unwrap();

        let canvas = document
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();

        canvas.style().set_css_text(&format!(
            "display:block; margin: auto; image-rendering: pixelated; width: {}px; height: {}px",
            size.w * 2,
            size.h * 2
        ));
        canvas.set_width(size.w as u32 * 2);
        canvas.set_height(size.h as u32 * 2);
    }
}
