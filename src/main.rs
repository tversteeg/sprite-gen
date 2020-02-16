use anyhow::Result;
use druid::kurbo::*;
use druid::lens::{Lens, LensWrap};
use druid::piet::*;
use druid::widget::*;
use druid::*;
use lazy_static::lazy_static;
use sprite_gen::{gen_sprite, MaskValue, Options};
use std::{convert::From, sync::RwLock};

const MAX_GRID_SIZE: usize = 128;
const MAX_SCALE: usize = 32;

const RECALCULATE_SPRITES: Selector = Selector::new("recalculate_sprites");

lazy_static! {
    static ref GRID: RwLock<Vec<MaskValue>> =
        RwLock::new(vec![MaskValue::Empty; MAX_GRID_SIZE * MAX_GRID_SIZE]);
    static ref RESULTS: RwLock<Vec<(usize, usize, Vec<u8>)>> = RwLock::new(Vec::new());
}

struct GridWidget {}

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

struct ResultWidget {}

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

                let result_width = if options.mirror_x { width * 2 } else { width };
                let result_height = if options.mirror_y { height * 2 } else { height };

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

pub trait MaskValueEx {
    fn color(&self) -> Color;
}

impl MaskValueEx for MaskValue {
    fn color(&self) -> Color {
        match self {
            MaskValue::Empty => Color::WHITE,
            MaskValue::Solid => Color::grey8(64),
            MaskValue::Body1 => Color::rgb8(255, 128, 128),
            MaskValue::Body2 => Color::rgb8(128, 128, 255),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Data, Lens)]
struct AppState {
    pub fill_type: i8,
    pub size_x: f64,
    pub size_y: f64,
    pub render_scale: f64,
    pub mirror_x: bool,
    pub mirror_y: bool,
}

impl AppState {
    // Size of each grid block
    pub fn block_size(&self, total_area: &Size) -> Size {
        Size::new(
            total_area.width / self.width() as f64,
            total_area.height / self.height() as f64,
        )
    }

    pub fn width(&self) -> usize {
        (self.size_x * MAX_GRID_SIZE as f64).floor().max(1.0) as usize
    }

    pub fn height(&self) -> usize {
        (self.size_y * MAX_GRID_SIZE as f64).floor().max(1.0) as usize
    }

    pub fn scale(&self) -> usize {
        (self.render_scale * MAX_SCALE as f64).floor().max(1.0) as usize
    }

    pub fn options(&self) -> Options {
        Options {
            mirror_x: self.mirror_x,
            mirror_y: self.mirror_y,
            ..Default::default()
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            size_x: 0.05,
            size_y: 0.05,
            render_scale: 0.2,
            mirror_x: true,
            mirror_y: false,
            fill_type: MaskValue::Solid.i8(),
        }
    }
}

fn ui_builder() -> impl Widget<AppState> {
    let fill_type = LensWrap::new(
        RadioGroup::new(vec![
            ("Solid", MaskValue::Solid.i8()),
            ("Empty", MaskValue::Empty.i8()),
            ("Body 1", MaskValue::Body1.i8()),
            ("Body 2", MaskValue::Body2.i8()),
        ]),
        AppState::fill_type,
    );
    let size_x = LensWrap::new(Slider::new(), AppState::size_x);
    let size_x_label =
        Label::new(|data: &AppState, _env: &_| format!("x pixels: {}", data.width()));
    let size_y = LensWrap::new(Slider::new(), AppState::size_y);
    let size_y_label =
        Label::new(|data: &AppState, _env: &_| format!("y pixels: {}", data.height()));
    let scale = LensWrap::new(Slider::new(), AppState::render_scale);
    let scale_label =
        Label::new(|data: &AppState, _env: &_| format!("render scale: {}", data.scale()));

    let options_box = {
        let mirror_x = LensWrap::new(Checkbox::new(), AppState::mirror_x);
        let mirror_x_label = Label::new("Mirror X");
        let mirror_y = LensWrap::new(Checkbox::new(), AppState::mirror_y);
        let mirror_y_label = Label::new("Mirror Y");
        Padding::new(
            20.0,
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_child(Padding::new(5.0, mirror_x), 0.0)
                        .with_child(Padding::new(5.0, mirror_x_label), 1.0),
                    0.0,
                )
                .with_child(
                    Flex::row()
                        .with_child(Padding::new(5.0, mirror_y), 0.0)
                        .with_child(Padding::new(5.0, mirror_y_label), 1.0),
                    0.0,
                ),
        )
    };

    Flex::column()
        .with_child(
            Padding::new(
                5.0,
                Flex::row()
                    .with_child(Padding::new(20.0, GridWidget::new_centered()), 1.0)
                    .with_child(
                        Flex::column()
                            .with_child(Padding::new(5.0, fill_type), 0.0)
                            .with_child(Padding::new(5.0, size_x), 0.0)
                            .with_child(size_x_label, 0.0)
                            .with_child(Padding::new(5.0, size_y), 0.0)
                            .with_child(size_y_label, 0.0)
                            .with_child(Padding::new(5.0, scale), 0.0)
                            .with_child(scale_label, 0.0)
                            .with_child(options_box, 1.0),
                        1.0,
                    ),
            ),
            1.0,
        )
        .with_child(Padding::new(20.0, ResultWidget::new_centered()), 0.5)
}

fn main_menu_builder<T: Data>() -> MenuDesc<T> {
    let mut base = MenuDesc::empty();
    #[cfg(target_os = "macos")]
    {
        base = druid::platform_menus::mac::menu_bar();
    }
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        base = base.append(druid::platform_menus::win::file::default());
    }

    base
}

fn main() -> Result<()> {
    let main_window = WindowDesc::new(ui_builder)
        .title(LocalizedString::new("Sprite"))
        .menu(main_menu_builder());

    let data = AppState::default();

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Could not create main window");

    Ok(())
}
