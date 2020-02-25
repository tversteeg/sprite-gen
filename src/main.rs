mod appstate;
mod widgets;

use crate::{appstate::*, widgets::*};
use anyhow::Result;
use druid::commands::*;
use druid::lens::LensWrap;
use druid::widget::*;
use druid::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use sprite_gen::MaskValue;
use std::fs::File;

const BOX_SIZE: f64 = 100.0;
const LABEL_SIZE: f64 = 200.0;

#[derive(Debug, Serialize, Deserialize)]
struct Encoded {
    pub state: AppState,
    pub grid: Vec<i8>,
}

struct Delegate {}

impl AppDelegate<AppState> for Delegate {
    fn event(
        &mut self,
        event: Event,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) -> Option<Event> {
        match event {
            Event::Command(cmd) => match cmd.selector {
                NEW_FILE => {
                    // Clear the grid
                    GRID.write()
                        .unwrap()
                        .iter_mut()
                        .for_each(|p| *p = MaskValue::Empty);

                    // Clear the results
                    RESULTS.write().unwrap().clear();

                    None
                }
                SAVE_FILE => {
                    // Get the file path from the Save As menu if applicable
                    if let Some(file_info) = cmd.get_object::<FileInfo>() {
                        data.file_path = Some(file_info.path().to_str().unwrap().to_string());
                    }

                    if data.file_path == None {
                        // There's no path yet, show the popup
                        return Some(Event::Command(SHOW_SAVE_PANEL.into()));
                    }

                    // Save the current grid to a new file
                    bincode::serialize_into(
                        File::create(data.file_path.as_ref().unwrap()).unwrap(),
                        &Encoded {
                            state: data.clone(),
                            // Convert the grid to an array of i8
                            grid: data.pixels().into_iter().map(|p| p.i8()).collect::<_>(),
                        },
                    )
                    .expect("Could not serialize or write to save file");

                    None
                }
                OPEN_FILE => {
                    if let Some(file_info) = cmd.get_object::<FileInfo>() {
                        data.file_path = Some(file_info.path().to_str().unwrap().to_string());
                    }

                    if data.file_path == None {
                        // There's no path yet, do nothing
                        return None;
                    }

                    let decoded: Encoded = bincode::deserialize_from(
                        File::open(data.file_path.as_ref().unwrap()).unwrap(),
                    )
                    .expect("Could not deserialize or open file");

                    *data = decoded.state;

                    None
                }
                _ => Some(Event::Command(cmd)),
            },

            other => Some(other),
        }
    }

    fn window_added(
        &mut self,
        _id: WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
    }

    fn window_removed(
        &mut self,
        _id: WindowId,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
    }
}

fn copy_to_clipboard(data: &AppState) {
    let string = format!(
        "let (width, height, options) = ({}, {}, {:?});\nlet data = [{}];",
        data.width(),
        data.height(),
        data.options(),
        data.pixels()
            .into_iter()
            .map(|p| format!("MaskValue::{:?}", p))
            .join(", ")
    );

    let mut clipboard = Application::clipboard();
    clipboard.put_string(string);
}

fn ui_builder() -> impl Widget<AppState> {
    let edit_box = {
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
            Label::new(|data: &AppState, _env: &_| format!("X Pixels: {}", data.width()));
        let size_y = LensWrap::new(Slider::new(), AppState::size_y);
        let size_y_label =
            Label::new(|data: &AppState, _env: &_| format!("Y Pixels: {}", data.height()));
        let scale = LensWrap::new(Slider::new(), AppState::render_scale);
        let scale_label =
            Label::new(|data: &AppState, _env: &_| format!("Render Scale: {}", data.scale()));
        let results_amount = LensWrap::new(Slider::new(), AppState::results_amount);
        let results_amount_label =
            Label::new(|data: &AppState, _env: &_| format!("Results: {}", data.results()));

        let right_box = Flex::column()
            .with_child(
                Flex::row()
                    .with_child(size_x.padding(5.0), 1.0)
                    .with_child(size_x_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_child(
                Flex::row()
                    .with_child(size_y.padding(5.0), 1.0)
                    .with_child(size_y_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_child(
                Flex::row()
                    .with_child(scale.padding(5.0), 1.0)
                    .with_child(scale_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_child(
                Flex::row()
                    .with_child(results_amount.padding(5.0), 1.0)
                    .with_child(results_amount_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            );

        Flex::row()
            .with_child(fill_type.fix_width(BOX_SIZE), 0.0)
            .with_child(right_box, 1.0)
    };

    let options_box = {
        let colored = LensWrap::new(Checkbox::new(), AppState::colored);
        let mirror_x = LensWrap::new(Checkbox::new(), AppState::mirror_x);
        let mirror_y = LensWrap::new(Checkbox::new(), AppState::mirror_y);
        let left_box = Flex::column()
            .with_child(
                Flex::row()
                    .with_child(colored.padding(5.0), 0.0)
                    .with_child(Label::new("Colored").padding(5.0), 0.0),
                0.0,
            )
            .with_child(
                Flex::row()
                    .with_child(mirror_x.padding(5.0), 0.0)
                    .with_child(Label::new("Mirror X").padding(5.0), 0.0),
                0.0,
            )
            .with_child(
                Flex::row()
                    .with_child(mirror_y.padding(5.0), 0.0)
                    .with_child(Label::new("Mirror Y").padding(5.0), 0.0),
                0.0,
            );

        let edge_brightness = LensWrap::new(Slider::new(), AppState::edge_brightness);
        let edge_brightness_label = Label::new(|data: &AppState, _env: &_| {
            format!("Edge Brightness: {:.2}", data.edge_brightness)
        });
        let color_variations = LensWrap::new(Slider::new(), AppState::color_variations);
        let color_variations_label = Label::new(|data: &AppState, _env: &_| {
            format!("Color Variations: {:.2}", data.color_variations)
        });
        let brightness_noise = LensWrap::new(Slider::new(), AppState::brightness_noise);
        let brightness_noise_label = Label::new(|data: &AppState, _env: &_| {
            format!("Brightness Noise: {:.2}", data.brightness_noise)
        });
        let saturation = LensWrap::new(Slider::new(), AppState::saturation);
        let saturation_label =
            Label::new(|data: &AppState, _env: &_| format!("Saturation: {:.2}", data.saturation));
        let right_box = Flex::column()
            .with_child(
                Flex::row()
                    .with_child(edge_brightness.padding(5.0), 1.0)
                    .with_child(edge_brightness_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_child(
                Flex::row()
                    .with_child(color_variations.padding(5.0), 1.0)
                    .with_child(color_variations_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_child(
                Flex::row()
                    .with_child(brightness_noise.padding(5.0), 1.0)
                    .with_child(brightness_noise_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_child(
                Flex::row()
                    .with_child(saturation.padding(5.0), 1.0)
                    .with_child(saturation_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            );

        Flex::row()
            .with_child(left_box.fix_width(BOX_SIZE), 0.0)
            .with_child(
                // Let the color options rendering state depend on the colored boolean
                Either::new(|data, _env| data.colored, right_box, SizedBox::empty()),
                1.0,
            )
    };

    let copy_to_clipboard_button = Button::new("Copy to clipboard", |_ctx, data, _env| {
        copy_to_clipboard(data)
    });

    Flex::column()
        .with_child(
            Flex::row()
                .with_child(GridWidget::new_centered().padding(20.0), 1.0)
                .with_child(
                    Flex::column()
                        .with_child(Label::new("Edit").padding(5.0), 0.0)
                        .with_child(edit_box, 1.0)
                        .with_child(Label::new("Options").padding(5.0), 0.0)
                        .with_child(options_box, 1.0)
                        .with_child(copy_to_clipboard_button.padding(5.0), 0.0),
                    1.0,
                )
                .padding(5.0),
            1.0,
        )
        .with_child(ResultWidget::new_centered().padding(20.0), 0.5)
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
        .delegate(Delegate {})
        .use_simple_logger()
        .launch(data)
        .expect("Could not create main window");

    Ok(())
}
