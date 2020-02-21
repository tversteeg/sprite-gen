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
                    .unwrap();

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

    let copy_to_clipboard_button = Button::new("Copy to clipboard", |_ctx, data, _env| {
        copy_to_clipboard(data)
    });

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
                            .with_child(options_box, 1.0)
                            .with_child(Padding::new(5.0, copy_to_clipboard_button), 0.0),
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
        .delegate(Delegate {})
        .use_simple_logger()
        .launch(data)
        .expect("Could not create main window");

    Ok(())
}
