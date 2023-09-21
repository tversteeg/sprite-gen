mod assets;
mod font;
mod input;
mod sprite;
mod widgets;
mod window;

use std::sync::OnceLock;

use assets::Assets;
use assets_manager::{loader::TomlLoader, Asset, AssetGuard, Compound};
use font::Font;
use input::Input;
use miette::Result;
use serde::Deserialize;
use sprite::Sprite;
#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Runtime;
use vek::{Aabr, Extent2, Vec2};
use widgets::{button::Button, grid::Grid, slider::Slider};

/// Window size.
pub const SIZE: Extent2<usize> = Extent2::new(640, 480);

/// The assets as a 'static reference.
pub static ASSETS: OnceLock<Assets> = OnceLock::new();

/// Application state.
#[derive(Debug, Default)]
struct State {
    /// Grid for drawing.
    drawing_area: Grid,
    /// Slider for X pixels value.
    x_pixels_slider: Slider,
    /// Slider for Y pixels value.
    y_pixels_slider: Slider,
    /// Button to clear the canvas.
    clear_canvas_button: Button,
}

impl State {
    /// Construct the initial state.
    pub fn new() -> Self {
        let settings = crate::settings();

        let drawing_area = Grid {
            offset: Vec2::new(260.0, 40.0),
            size: Vec2::new(settings.min_x_pixels, settings.min_y_pixels).as_(),
            scaling: Vec2::new(16, 16),
            ..Default::default()
        };

        let x_pixels_slider = Slider {
            offset: Vec2::new(20.0, 40.0),
            length: 100.0,
            value_label: Some("X Pixels".to_string()),
            min: settings.min_x_pixels,
            max: settings.max_x_pixels,
            steps: Some((settings.max_x_pixels - settings.min_x_pixels) / 4.0),
            ..Default::default()
        };

        let y_pixels_slider = Slider {
            offset: Vec2::new(20.0, 70.0),
            length: 100.0,
            min: settings.min_y_pixels,
            max: settings.max_y_pixels,
            value_label: Some("Y Pixels".to_string()),
            steps: Some((settings.max_y_pixels - settings.min_y_pixels) / 4.0),
            ..Default::default()
        };

        let clear_canvas_button = Button {
            offset: Vec2::new(10.0, 10.0),
            size: Extent2::new(80.0, 18.0),
            label: Some("Clear".to_string()),
            ..Default::default()
        };

        Self {
            drawing_area,
            x_pixels_slider,
            y_pixels_slider,
            clear_canvas_button,
        }
    }

    /// Update application state and handle input.
    pub fn update(&mut self, input: &Input) {
        if self.x_pixels_slider.update(input) || self.y_pixels_slider.update(input) {
            self.drawing_area.resize(
                Vec2::new(self.x_pixels_slider.value(), self.y_pixels_slider.value()).as_(),
                Vec2::new(
                    if self.x_pixels_slider.value() < 24.0 {
                        16
                    } else {
                        9
                    },
                    if self.y_pixels_slider.value() < 24.0 {
                        16
                    } else {
                        9
                    },
                ),
            );
        }
        self.drawing_area.update(input);

        self.clear_canvas_button.update(input);
    }

    /// Render the window.
    pub fn render(&self, canvas: &mut [u32]) {
        font().render_centered(
            "Sprite Generation",
            Vec2::new(SIZE.w as f64 / 2.0, 10.0),
            canvas,
        );

        self.drawing_area.render(canvas);
        self.x_pixels_slider.render(canvas);
        self.y_pixels_slider.render(canvas);
        self.clear_canvas_button.render(canvas);
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
