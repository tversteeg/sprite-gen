use anyhow::Result;
use druid::{
    lens::{Lens, LensWrap},
    platform_menus::common::{copy, cut, paste},
    widget::{Flex, Label, Padding, RadioGroup, Slider},
    AppLauncher, Data, LocalizedString, MenuDesc, Widget, WindowDesc,
};

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

    Flex::column().with_child(
        Flex::row()
            .with_child(Padding::new(5.0, fill_type), 0.0)
            .with_child(
                Flex::column()
                    .with_child(Padding::new(5.0, size_x), 0.0)
                    .with_child(Padding::new(0.0, size_x_label), 0.0)
                    .with_child(Padding::new(5.0, size_y), 0.0)
                    .with_child(Padding::new(0.0, size_y_label), 0.0),
                1.0,
            ),
        0.0,
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
        .title(LocalizedString::new("Shimmer"))
        .menu(main_menu_builder());

    let data = AppState::default();

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Could not create main window");

    Ok(())
}
