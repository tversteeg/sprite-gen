use crate::appstate::*;

use druid::kurbo::*;

use druid::piet::*;
use druid::widget::*;
use druid::*;

use sprite_gen::{gen_sprite, MaskValue};
use std::convert::From;

pub const RECALCULATE_SPRITES: Selector = Selector::new("recalculate_sprites");

pub struct GridWidget {}

impl GridWidget {
    pub fn new_centered() -> impl Widget<AppState> {
        Align::centered(Self {})
    }
}

impl Widget<AppState> for GridWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        if let Event::MouseDown(mouse) = event {
            let size = ctx.size();
            let block_size = data.block_size(&size);

            let index_x = (mouse.pos.x / block_size.width).floor() as usize;
            let index_y = (mouse.pos.y / block_size.height).floor() as usize;

            GRID.write().unwrap()[index_y * MAX_GRID_SIZE + index_x] = From::from(data.fill_type);

            ctx.submit_command(RECALCULATE_SPRITES, None);

            // Force a redraw of the grid
            ctx.invalidate();
        }
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: Option<&AppState>,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.debug_check("Grid");

        bc.max()
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let size = paint_ctx.size();

        let block_size = data.block_size(&size);

        let grid = GRID.read().unwrap();

        for y_pixels in 0..data.height() {
            for x_pixels in 0..data.width() {
                let offset = Point::new(
                    x_pixels as f64 * block_size.width,
                    y_pixels as f64 * block_size.height,
                );

                let rect = Rect::from_origin_size(offset, block_size);

                let color = grid[x_pixels + y_pixels * MAX_GRID_SIZE].color();

                paint_ctx.stroke(rect, &env.get(theme::BORDER_LIGHT), 2.0);

                paint_ctx.fill(rect, &color);
            }
        }
    }
}

pub struct ResultWidget {}

impl ResultWidget {
    pub fn new_centered() -> impl Widget<AppState> {
        Align::centered(Self {})
    }
}

impl Widget<AppState> for ResultWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, _env: &Env) {
        if let Event::Command(cmd) = event {
            if cmd.selector == RECALCULATE_SPRITES {
                // Generate new sprites

                let width = data.width();
                let height = data.height();

                // Copy the mask
                let mask = {
                    let grid = GRID.read().unwrap();

                    let mut new = vec![MaskValue::default(); width * height];

                    for y_pixels in 0..height {
                        for x_pixels in 0..width {
                            new[y_pixels * width + x_pixels] =
                                grid[y_pixels * MAX_GRID_SIZE + x_pixels].clone();
                        }
                    }

                    new
                };

                let mut results = RESULTS.write().unwrap();

                results.clear();

                let options = data.options();

                let result_width = data.result_width();
                let result_height = data.result_height();

                for _ in 0..100 {
                    results.push((
                        result_width,
                        result_height,
                        gen_sprite(&mask, width, options)
                            // Convert Vec<u32> to a Vec<u8>
                            .into_iter()
                            .map(|p| {
                                vec![
                                    ((p >> 16) & 0xFF) as u8,
                                    ((p >> 8) & 0xFF) as u8,
                                    (p & 0xFF) as u8,
                                ]
                            })
                            .flatten()
                            .collect::<_>(),
                    ));
                }

                ctx.invalidate();
            }
        }
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: Option<&AppState>,
        _data: &AppState,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.debug_check("Result");

        bc.max()
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let ctx_size = paint_ctx.size();

        let rect = Rect::from_origin_size(Point::ORIGIN, ctx_size);

        paint_ctx.stroke(rect, &env.get(theme::BORDER_LIGHT), 2.0);

        paint_ctx.fill(rect, &MaskValue::Empty.color());

        let scale = data.scale();
        let padding = 4;

        // Make the background white

        // Render the results
        let mut x = 0;
        let mut y = 0;
        for (width, height, result) in RESULTS.read().unwrap().iter() {
            let size = Size::new((width * scale) as f64, (height * scale) as f64);

            let image = paint_ctx
                .make_image(*width, *height, &result, ImageFormat::Rgb)
                .unwrap();
            // The image is automatically scaled to fit the rect you pass to draw_image
            paint_ctx.draw_image(
                &image,
                Rect::from_origin_size(Point::new(x as f64, y as f64), size),
                InterpolationMode::NearestNeighbor,
            );

            x += width * scale + padding;
            if x as f64 + size.width > ctx_size.width {
                x = 0;
                y += height * scale + padding;
            }
        }
    }
}
