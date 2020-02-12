use anyhow::Result;
use druid::kurbo::*;
use druid::lens::{Lens, LensWrap};
use druid::piet::*;
use druid::platform_menus::common::{copy, cut, paste};
use druid::widget::*;
use druid::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref GRID: Vec<FillType> = vec![FillType::Empty; 1024 * 1024];
}

struct GridWidget {}

impl GridWidget {
    pub fn new_centered() -> impl Widget<AppState> {
        Align::centered(Self {})
    }
}

impl Widget<AppState> for GridWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}

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

        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        bc.max()
    }

    // The paint method gets called last, after an event flow.
    // It goes event -> update -> layout -> paint, and each method can influence the next.
    // Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, paint_ctx: &mut PaintCtx, _data: &AppState, env: &Env) {
        let rect = Rect::from_origin_size(Point::ORIGIN, paint_ctx.size());

        paint_ctx.stroke(rect, &env.get(theme::BORDER), 2.0);

        paint_ctx.fill(rect, &env.get(theme::BACKGROUND_LIGHT));

        // Let's burn some CPU to make a (partially transparent) image buffer
        /*
        let image_data = make_image_data(256, 256);
        let image = paint_ctx
            .make_image(256, 256, &image_data, ImageFormat::RgbaSeparate)
            .unwrap();
        // The image is automatically scaled to fit the rect you pass to draw_image
        paint_ctx.draw_image(
            &image,
            Rect::from_origin_size(Point::ORIGIN, size),
            InterpolationMode::Bilinear,
        );
        */
    }
}

#[derive(Clone, Eq, PartialEq, Data)]
enum FillType {
    Solid,
    Empty,
    Color1,
    Color2,
}

impl Default for FillType {
    fn default() -> Self {
        FillType::Solid
    }
}

#[derive(Clone, Default, PartialEq, Data, Lens)]
struct AppState {
    pub fill_type: FillType,
    pub size_x: f64,
    pub size_y: f64,
}

fn ui_builder() -> impl Widget<AppState> {
    let fill_type = LensWrap::new(
        RadioGroup::new(vec![
            ("Solid", FillType::Solid),
            ("Empty", FillType::Empty),
            ("Color 1", FillType::Color1),
            ("Color 2", FillType::Color2),
        ]),
        AppState::fill_type,
    );
    let size_x = LensWrap::new(Slider::new(), AppState::size_x);
    let size_x_label =
        Label::new(|data: &AppState, _env: &_| format!("x: {0:.0}", data.size_x * 1024.0));
    let size_y = LensWrap::new(Slider::new(), AppState::size_y);
    let size_y_label =
        Label::new(|data: &AppState, _env: &_| format!("y: {0:.0}", data.size_y * 1024.0));

    Flex::row()
        .with_child(Padding::new(5.0, fill_type), 0.0)
        .with_child(
            Flex::column()
                .with_child(Padding::new(5.0, size_x), 0.0)
                .with_child(Padding::new(0.0, size_x_label), 0.0)
                .with_child(Padding::new(5.0, size_y), 0.0)
                .with_child(Padding::new(0.0, size_y_label), 0.0)
                .with_child(Padding::new(20.0, GridWidget::new_centered()), 1.0),
            1.0,
        )
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

    base.append(
        MenuDesc::new(LocalizedString::new("common-menu-edit-menu"))
            .append(cut())
            .append(copy())
            .append(paste()),
    )
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
