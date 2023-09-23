mod assets;
mod font;
mod input;
mod sprite;
mod sprites;
mod widgets;
mod window;

use std::sync::OnceLock;

use assets::Assets;
use assets_manager::{loader::TomlLoader, Asset, AssetGuard};
use font::Font;
use input::Input;
use miette::Result;
use serde::Deserialize;
use sprite::Sprite;
use sprite_gen::{MaskValue, Options};
use sprites::Sprites;
use taffy::{
    prelude::{Node, Rect, Size},
    style::{
        AlignContent, AlignItems, AvailableSpace, Dimension, Display, FlexDirection, FlexWrap,
        Style,
    },
    style_helpers::TaffyMaxContent,
    tree::LayoutTree,
    Taffy,
};
#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Runtime;
use vek::{Extent2, Vec2};
use widgets::{button::Button, checkbox::CheckboxGroup, grid::Grid, radio::Radio, slider::Slider};

/// Window size.
pub const SIZE: Extent2<usize> = Extent2::new(640, 640);

/// The assets as a 'static reference.
pub static ASSETS: OnceLock<Assets> = OnceLock::new();

/// Application state.
struct State {
    /// Rendered sprites.
    sprites: Sprites,
    /// Grid for drawing.
    drawing_area: Grid,
    /// Slider for X pixels value.
    x_pixels_slider: Slider,
    /// Slider for Y pixels value.
    y_pixels_slider: Slider,
    /// Button to clear the canvas.
    clear_canvas_button: Button,
    /// Radio button group for the brush.
    brush_radio: Radio<4>,
    /// Options checkbox group.
    options_group: CheckboxGroup<3>,
    /// Selected brush type.
    brush: MaskValue,
    /// Slider for edge brightness.
    edge_brightness_slider: Slider,
    /// Slider for color variations.
    color_variations_slider: Slider,
    /// Slider for brightness noise.
    brightness_noise_slider: Slider,
    /// Slider for saturation.
    saturation_slider: Slider,
    /// Flexbox grid to lay out the widgets.
    layout: Taffy,
    /// Root grid node.
    root: Node,
}

impl State {
    /// Construct the initial state.
    pub fn new() -> Self {
        let settings = crate::settings();

        // Define the layout
        let mut layout = Taffy::new();

        // Grid for editing the sprite shape
        let drawing_area = Grid::new(
            layout
                .new_leaf(Style {
                    size: Size::auto(),
                    justify_content: Some(AlignContent::Center),
                    min_size: Size::from_points(200.0, 200.0),
                    ..Default::default()
                })
                .unwrap(),
            Extent2::new(settings.min_x_pixels, settings.min_y_pixels).as_(),
        );

        let slider_style = Style {
            size: Size::from_points(250.0, 20.0),
            margin: Rect {
                left: taffy::style_helpers::points(5.0),
                right: taffy::style_helpers::auto(),
                top: taffy::style_helpers::auto(),
                bottom: taffy::style_helpers::auto(),
            },
            ..Default::default()
        };
        let x_pixels_slider = Slider {
            node: layout.new_leaf(slider_style.clone()).unwrap(),
            length: 100.0,
            value_label: Some("X Pixels".to_string()),
            min: settings.min_x_pixels,
            max: settings.max_x_pixels,
            steps: Some((settings.max_x_pixels - settings.min_x_pixels) / 4.0),
            ..Default::default()
        };

        let y_pixels_slider = Slider {
            node: layout.new_leaf(slider_style.clone()).unwrap(),
            length: 100.0,
            min: settings.min_y_pixels,
            max: settings.max_y_pixels,
            value_label: Some("Y Pixels".to_string()),
            steps: Some((settings.max_y_pixels - settings.min_y_pixels) / 4.0),
            ..Default::default()
        };

        let button_style = Style {
            size: Size::from_points(80.0, 18.0),
            ..Default::default()
        };
        let clear_canvas_button = Button {
            node: layout.new_leaf(button_style.clone()).unwrap(),
            label: Some("Clear".to_string()),
            ..Default::default()
        };

        let brush_radio = Radio::new(
            ["Solid", "Empty", "Body1", "Body2"],
            Some("Brush".to_string()),
            0,
            layout
                .new_leaf(Style {
                    min_size: Size::from_points(250.0, 150.0),
                    ..Default::default()
                })
                .unwrap(),
        );
        let brush = MaskValue::Solid;

        let options_group = CheckboxGroup::new(
            [("Colored", true), ("Mirror X", true), ("Mirror Y", false)],
            Some("Options".to_string()),
            layout
                .new_leaf(Style {
                    min_size: Size::from_points(250.0, 120.0),
                    ..Default::default()
                })
                .unwrap(),
        );

        let edge_brightness_slider = Slider {
            node: layout.new_leaf(slider_style.clone()).unwrap(),
            length: 100.0,
            value_label: Some("Edge Brightness".to_string()),
            min: 0.0,
            max: 100.0,
            pos: 0.17,
            ..Default::default()
        };

        let color_variations_slider = Slider {
            node: layout.new_leaf(slider_style.clone()).unwrap(),
            length: 100.0,
            value_label: Some("Color Variations".to_string()),
            min: 0.0,
            max: 100.0,
            pos: 0.2,
            ..Default::default()
        };

        let brightness_noise_slider = Slider {
            node: layout.new_leaf(slider_style.clone()).unwrap(),
            length: 100.0,
            value_label: Some("Brightness Noise".to_string()),
            min: 0.0,
            max: 100.0,
            pos: 0.81,
            ..Default::default()
        };

        let saturation_slider = Slider {
            node: layout.new_leaf(slider_style.clone()).unwrap(),
            length: 100.0,
            value_label: Some("Saturation".to_string()),
            min: 0.0,
            max: 100.0,
            pos: 0.54,
            ..Default::default()
        };

        let sprites = Sprites {
            offset: Vec2::new(5.0, 470.0),
            size: Extent2::new(
                x_pixels_slider.value() as usize,
                y_pixels_slider.value() as usize,
            ),
            amount: settings.preview_requested,
            ..Default::default()
        };

        let gap = Size {
            width: taffy::style_helpers::points(2.0),
            height: taffy::style_helpers::points(2.0),
        };

        // Split the layout top vertical part into two horizontal parts
        let topleft = layout
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: Some(AlignContent::SpaceAround),
                    gap,
                    ..Default::default()
                },
                &[
                    clear_canvas_button.node,
                    x_pixels_slider.node,
                    y_pixels_slider.node,
                    brush_radio.node,
                    options_group.node,
                    edge_brightness_slider.node,
                    saturation_slider.node,
                    color_variations_slider.node,
                    brightness_noise_slider.node,
                ],
            )
            .unwrap();
        let topright = layout
            .new_with_children(
                Style {
                    flex_grow: 1.0,
                    min_size: Size::from_percent(0.5, 0.5),
                    gap,
                    ..Default::default()
                },
                &[drawing_area.node],
            )
            .unwrap();

        // Split the layout into two vertical parts
        let top = layout
            .new_with_children(
                Style {
                    min_size: Size {
                        width: taffy::style_helpers::percent(1.0),
                        height: taffy::style_helpers::auto(),
                    },
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: Some(AlignContent::SpaceBetween),
                    align_items: Some(AlignItems::Stretch),
                    gap,
                    ..Default::default()
                },
                &[topleft, topright],
            )
            .unwrap();
        let bottom = layout
            .new_leaf(Style {
                min_size: Size {
                    width: taffy::style_helpers::percent(1.0),
                    height: taffy::style_helpers::percent(0.3),
                },
                gap,
                ..Default::default()
            })
            .unwrap();

        // Everything together
        let root = layout
            .new_with_children(
                Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: Some(AlignContent::Center),
                    size: Size::from_points(SIZE.w as f32, SIZE.h as f32),
                    padding: Rect::points(5.0),
                    gap,
                    ..Default::default()
                },
                &[top, bottom],
            )
            .unwrap();

        let mut this = Self {
            sprites,
            drawing_area,
            x_pixels_slider,
            y_pixels_slider,
            clear_canvas_button,
            brush_radio,
            options_group,
            brush,
            edge_brightness_slider,
            color_variations_slider,
            brightness_noise_slider,
            saturation_slider,
            layout,
            root,
        };

        this.update_layout();
        this.generate();

        this
    }

    /// Update application state and handle input.
    pub fn update(&mut self, input: &Input) {
        if self.x_pixels_slider.update(input) || self.y_pixels_slider.update(input) {
            let x_pixels = self.x_pixels_slider.value();
            let y_pixels = self.y_pixels_slider.value();
            // Resize the drawing area
            self.drawing_area.resize(
                Extent2::new(x_pixels, y_pixels).as_(),
                Extent2::new(
                    if x_pixels == 4.0 {
                        64
                    } else if x_pixels < 12.0 {
                        32
                    } else if x_pixels < 24.0 {
                        16
                    } else {
                        9
                    },
                    if y_pixels == 4.0 {
                        64
                    } else if y_pixels < 12.0 {
                        32
                    } else if y_pixels < 24.0 {
                        16
                    } else {
                        9
                    },
                ),
            );

            // Resize the sprite results
            self.sprites.resize(
                Extent2::new(self.x_pixels_slider.value(), self.y_pixels_slider.value()).as_(),
            );

            self.generate();
        }

        // Allow user to draw
        if self.drawing_area.update(input, self.brush.clone()) {
            self.generate();
        }

        if self.clear_canvas_button.update(input) {
            self.drawing_area.clear();

            self.generate();
        }

        // Update the brush according to the radio group
        if let Some(selected) = self.brush_radio.update(input) {
            self.brush = match selected {
                0 => MaskValue::Solid,
                1 => MaskValue::Empty,
                2 => MaskValue::Body1,
                3 => MaskValue::Body2,
                _ => panic!(),
            };
        }

        if self.options_group.update(input).is_some() {
            self.generate();
        }

        if self.edge_brightness_slider.update(input)
            || self.color_variations_slider.update(input)
            || self.brightness_noise_slider.update(input)
            || self.saturation_slider.update(input)
        {
            self.generate();
        }
    }

    /// Render the window.
    pub fn render(&self, canvas: &mut [u32]) {
        self.drawing_area.render(canvas);
        self.x_pixels_slider.render(canvas);
        self.y_pixels_slider.render(canvas);
        self.clear_canvas_button.render(canvas);
        self.brush_radio.render(canvas);
        self.options_group.render(canvas);
        self.sprites.render(canvas);
        self.edge_brightness_slider.render(canvas);
        self.color_variations_slider.render(canvas);
        self.brightness_noise_slider.render(canvas);
        self.saturation_slider.render(canvas);
    }

    /// Update the layout.
    pub fn update_layout(&mut self) {
        // Compute the layout
        self.layout
            .compute_layout(self.root, Size::MAX_CONTENT)
            .unwrap();

        self.drawing_area.update_layout(
            self.abs_location(self.drawing_area.node),
            self.layout.layout(self.drawing_area.node).unwrap(),
        );
        self.x_pixels_slider
            .update_layout(self.abs_location(self.x_pixels_slider.node));
        self.y_pixels_slider
            .update_layout(self.abs_location(self.y_pixels_slider.node));
        self.clear_canvas_button.update_layout(
            self.abs_location(self.clear_canvas_button.node),
            self.layout.layout(self.clear_canvas_button.node).unwrap(),
        );
        self.brush_radio
            .update_layout(self.abs_location(self.brush_radio.node));
        self.options_group
            .update_layout(self.abs_location(self.options_group.node));
        self.edge_brightness_slider
            .update_layout(self.abs_location(self.edge_brightness_slider.node));
        self.saturation_slider
            .update_layout(self.abs_location(self.saturation_slider.node));
        self.color_variations_slider
            .update_layout(self.abs_location(self.color_variations_slider.node));
        self.brightness_noise_slider
            .update_layout(self.abs_location(self.brightness_noise_slider.node));

        taffy::debug::print_tree(&self.layout, self.root);
    }

    /// Generate new sprites.
    pub fn generate(&mut self) {
        // Scale to fill the rectangle with the lowest factor
        let area = Extent2::new(SIZE.w - 10, SIZE.h - self.sprites.offset.y as usize - 10);
        let width = self.x_pixels_slider.value() as usize
            * if self.options_group.checked(1) { 2 } else { 1 }
            + 4;
        let x_factor = area.w / width / settings().preview_requested.w;
        let height = self.y_pixels_slider.value() as usize
            * if self.options_group.checked(2) { 2 } else { 1 }
            + 4;
        let y_factor = area.h / height / settings().preview_requested.h;
        let scale = x_factor.min(y_factor).max(2);

        // Amount that can actually fit with the current size
        let amount = Extent2::new(area.w / width / scale, area.h / height / scale);

        // Redraw all sprites
        self.sprites.generate(
            self.drawing_area.mask(),
            Options {
                colored: self.options_group.checked(0),
                mirror_x: self.options_group.checked(1),
                mirror_y: self.options_group.checked(2),
                edge_brightness: self.edge_brightness_slider.value() as f32 / 100.0,
                color_variations: self.color_variations_slider.value() as f32 / 100.0,
                brightness_noise: self.brightness_noise_slider.value() as f32 / 100.0,
                saturation: self.saturation_slider.value() as f32 / 100.0,
                ..Default::default()
            },
            amount,
            scale,
        );
    }

    /// Get absolute coordinates for a node.
    ///
    /// We have to do this recursively because taffy doesn't expose it directly.
    pub fn abs_location(&self, mut node: Node) -> Vec2<f64> {
        let layout = self.layout.layout(node).unwrap().location;
        let mut coord = Vec2::new(layout.x as f64, layout.y as f64);

        while let Some(parent) = self.layout.parent(node) {
            let layout = self.layout.layout(parent).unwrap().location;
            coord.x += layout.x as f64;
            coord.y += layout.y as f64;

            node = parent;
        }

        coord
    }
}

/// Application settings loaded from a file so it's easier to change them with hot-reloading.
#[derive(Deserialize)]
pub struct Settings {
    /// Minimum amount of X pixels.
    min_x_pixels: f64,
    /// Maximum amount of X pixels.
    max_x_pixels: f64,
    /// Minimum amount of Y pixels.
    min_y_pixels: f64,
    /// Maximum amount of Y pixels.
    max_y_pixels: f64,
    /// Ideal amount of preview images.
    preview_requested: Extent2<usize>,
}

impl Asset for Settings {
    const EXTENSION: &'static str = "toml";

    type Loader = TomlLoader;
}

async fn run() -> Result<()> {
    // Initialize the asset loader
    let assets = ASSETS.get_or_init(Assets::load);
    assets.enable_hot_reloading();

    // Run the application window
    window::run(
        State::new(),
        SIZE,
        60,
        |g, input| {
            // Update the application state
            g.update(input);
        },
        |g, buffer| {
            // Clear with gray
            buffer.fill(0xFF999999);

            // Draw the application
            g.render(buffer);
        },
    )
    .await?;

    Ok(())
}

/// Entry point starting either a WASM future or a Tokio runtime.
fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("error initializing logger");

        wasm_bindgen_futures::spawn_local(async { run().await.unwrap() });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let rt = Runtime::new().unwrap();
        rt.block_on(async { run().await.unwrap() });
    }
}

/// Load the global settings.
pub fn settings() -> AssetGuard<'static, Settings> {
    ASSETS
        .get()
        .expect("Asset handling not initialized yet")
        .settings()
}

/// Load the font.
pub fn font() -> AssetGuard<'static, Font> {
    ASSETS
        .get()
        .expect("Asset handling not initialized yet")
        .asset("Beachball")
}

/// Load the sprite.
pub fn sprite(path: &str) -> AssetGuard<'static, Sprite> {
    ASSETS
        .get()
        .expect("Asset handling not initialized yet")
        .asset(path)
}
