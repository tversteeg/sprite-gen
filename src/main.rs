mod appstate;
mod widgets;

use crate::{appstate::*, widgets::*};
use anyhow::Result;
use druid::commands::*;
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
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> bool {
        if let Some(_) = cmd.get(NEW_FILE) {
            // Clear the grid
            GRID.write()
                .unwrap()
                .iter_mut()
                .for_each(|p| *p = MaskValue::Empty);

            // Clear the results
            RESULTS.write().unwrap().clear();

            return true;
        }
        if let Some(file_info) = cmd.get(OPEN_FILE) {
            data.file_path = Some(file_info.path().to_str().unwrap().to_string());

            let decoded: Encoded =
                bincode::deserialize_from(File::open(data.file_path.as_ref().unwrap()).unwrap())
                    .expect("Could not deserialize or open file");

            *data = decoded.state;

            return true;
        }
        if let Some(file_info) = cmd.get(SAVE_FILE) {
            // Get the file path from the Save As menu if applicable
            if let Some(file_info) = file_info {
                data.file_path = Some(file_info.path().to_str().unwrap().to_string());
            }

            if data.file_path == None {
                // There's no path yet, show the popup
                // TODO show save file popup
                return true;
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

            return true;
        }

        false
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

    let mut clipboard = Application::global().clipboard();
    clipboard.put_string(string);
}

fn ui_builder() -> impl Widget<AppState> {
    let menu_bar = {
        Flex::row()
            .with_child(
                Button::new("New")
                    .on_click(|ctx, _data, _env| ctx.submit_command(NEW_FILE, None))
                    .padding(5.0),
            )
            .with_child(
                Button::new("Open")
                    .on_click(|ctx, _data, _env| {
                        let dialog_options = FileDialogOptions::new();

                        ctx.submit_command(
                            Command::new(SHOW_OPEN_PANEL, dialog_options.clone()),
                            None,
                        )
                    })
                    .padding(5.0),
            )
            .with_child(
                Button::new("Save")
                    .on_click(|ctx, _data, _env| {
                        let dialog_options = FileDialogOptions::new();

                        ctx.submit_command(
                            Command::new(SHOW_SAVE_PANEL, dialog_options.clone()),
                            None,
                        )
                    })
                    .padding(5.0),
            )
            .with_child(
                Button::new("Copy to clipboard")
                    .on_click(|_ctx, data, _env| copy_to_clipboard(data))
                    .padding(5.0),
            )
    };

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

        let size_x = Slider::new().lens(AppState::size_x);
        let size_x_label =
            Label::new(|data: &AppState, _env: &_| format!("X Pixels: {}", data.width()));
        let size_y = Slider::new().lens(AppState::size_y);
        let size_y_label =
            Label::new(|data: &AppState, _env: &_| format!("Y Pixels: {}", data.height()));
        let scale = Slider::new()
            .with_range(1.0, 32.0)
            .lens(AppState::render_scale);
        let scale_label = Label::new(|data: &AppState, _env: &_| {
            format!("Render Scale: {:.0}", data.render_scale)
        });
        let results_amount = Slider::new().lens(AppState::results_amount);
        let results_amount_label =
            Label::new(|data: &AppState, _env: &_| format!("Results: {}", data.results()));

        let right_box = Flex::column()
            .with_flex_child(
                Flex::row()
                    .with_flex_child(size_x.padding(5.0), 1.0)
                    .with_flex_child(size_x_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_flex_child(
                Flex::row()
                    .with_flex_child(size_y.padding(5.0), 1.0)
                    .with_flex_child(size_y_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_flex_child(
                Flex::row()
                    .with_flex_child(scale.padding(5.0), 1.0)
                    .with_flex_child(scale_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_flex_child(
                Flex::row()
                    .with_flex_child(results_amount.padding(5.0), 1.0)
                    .with_flex_child(results_amount_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            );

        Flex::row()
            .with_flex_child(fill_type.fix_width(BOX_SIZE), 0.0)
            .with_flex_child(right_box, 1.0)
    };

    let options_box = {
        let colored = Checkbox::new("Colored").lens(AppState::colored);
        let mirror_x = Checkbox::new("Mirror X").lens(AppState::mirror_x);
        let mirror_y = Checkbox::new("Mirror Y").lens(AppState::mirror_y);
        let left_box = Flex::column()
            .with_flex_child(colored.padding(5.0), 0.0)
            .with_flex_child(mirror_x.padding(5.0), 0.0)
            .with_flex_child(mirror_y.padding(5.0), 0.0);

        let edge_brightness = Slider::new().lens(AppState::edge_brightness);
        let edge_brightness_label = Label::new(|data: &AppState, _env: &_| {
            format!("Edge Brightness: {:.2}", data.edge_brightness)
        });
        let color_variations = Slider::new().lens(AppState::color_variations);
        let color_variations_label = Label::new(|data: &AppState, _env: &_| {
            format!("Color Variations: {:.2}", data.color_variations)
        });
        let brightness_noise = Slider::new().lens(AppState::brightness_noise);
        let brightness_noise_label = Label::new(|data: &AppState, _env: &_| {
            format!("Brightness Noise: {:.2}", data.brightness_noise)
        });
        let saturation = Slider::new().lens(AppState::saturation);
        let saturation_label =
            Label::new(|data: &AppState, _env: &_| format!("Saturation: {:.2}", data.saturation));
        let right_box = Flex::column()
            .with_flex_child(
                Flex::row()
                    .with_flex_child(edge_brightness.padding(5.0), 1.0)
                    .with_flex_child(edge_brightness_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_flex_child(
                Flex::row()
                    .with_flex_child(color_variations.padding(5.0), 1.0)
                    .with_flex_child(color_variations_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_flex_child(
                Flex::row()
                    .with_flex_child(brightness_noise.padding(5.0), 1.0)
                    .with_flex_child(brightness_noise_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            )
            .with_flex_child(
                Flex::row()
                    .with_flex_child(saturation.padding(5.0), 1.0)
                    .with_flex_child(saturation_label.fix_width(LABEL_SIZE), 0.0),
                0.0,
            );

        Flex::row()
            .with_flex_child(left_box.fix_width(BOX_SIZE), 0.0)
            .with_flex_child(
                // Let the color options rendering state depend on the colored boolean
                Either::new(|data, _env| data.colored, right_box, SizedBox::empty()),
                1.0,
            )
    };

    Flex::column()
        .with_child(menu_bar.padding(5.0))
        .with_flex_child(
            Flex::row()
                .with_flex_child(GridWidget::new_centered().padding(20.0), 1.0)
                .with_flex_child(
                    Flex::column()
                        .with_flex_child(Label::new("Edit").padding(5.0), 0.0)
                        .with_flex_child(edit_box, 1.0)
                        .with_flex_child(Label::new("Options").padding(5.0), 0.0)
                        .with_flex_child(options_box, 1.0),
                    1.0,
                )
                .padding(5.0),
            1.0,
        )
        .with_flex_child(ResultWidget::new_centered().padding(20.0), 0.5)
}

fn main() -> Result<()> {
    let main_window = WindowDesc::new(ui_builder).title(LocalizedString::new("Sprite"));

    let data = AppState::default();

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .use_simple_logger()
        .launch(data)
        .expect("Could not create main window");

    Ok(())
}
