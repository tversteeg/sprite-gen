use blit::slice::Slice;
use vek::{Aabr, Extent2, Rect, Vec2};

use crate::input::Input;

/// A simple slider widget.
#[derive(Debug)]
pub struct Slider {
    /// Top-left position of the widget in pixels.
    pub offset: Vec2<f64>,
    /// Length of the slider in pixels.
    pub length: f64,
    /// Minimum value of the slider.
    pub min: f64,
    /// Maximum value of the slider.
    pub max: f64,
    /// How many steps the slider should snap to.
    pub steps: Option<f64>,
    /// Current positition of the slider as a fraction.
    pub pos: f64,
    /// Whether the slider state is being captured by the mouse.
    pub dragged: bool,
    /// A custom label with the value.
    pub value_label: Option<String>,
}

impl Slider {
    /// Handle the input.
    pub fn update(&mut self, input: &Input) {
        if !self.dragged {
            // Detect whether the mouse is being pressed on the handle
            let handle = crate::sprite("slider-handle");
            let handle_rect = Rect::new(
                self.offset.x - handle.width() as f64 / 2.0,
                self.offset.y - handle.height() as f64 / 2.0,
                handle.width() as f64 * 2.0 + self.length,
                handle.height() as f64 * 2.0,
            );

            if !self.dragged
                && input.left_mouse.is_pressed()
                && handle_rect.contains_point(input.mouse_pos.as_())
            {
                self.dragged = true;
            }
        } else if input.left_mouse.is_released() {
            // Always release the slider when the mouse is released
            self.dragged = false;
        } else if input.left_mouse.is_down() {
            // Drag the slider
            self.pos = ((input.mouse_pos.x as f64 - self.offset.x) / self.length).clamp(0.0, 1.0);
        }
    }

    /// Render the slider.
    pub fn render(&self, canvas: &mut [u32]) {
        let handle = crate::sprite("slider-handle");
        let bar = crate::sprite("slider-bar");
        bar.render_vertical_slice(
            canvas,
            self.offset
                + (
                    0.0,
                    handle.height() as f64 / 2.0 - bar.height() as f64 / 2.0,
                ),
            self.length,
            Slice::Ternary {
                split_first: 2,
                split_last: 3,
            },
        );
        handle.render(
            canvas,
            self.offset
                + (
                    self.pos.clamp(0.0, 1.0) * self.length - handle.width() as f64 / 2.0,
                    0.0,
                ),
        );

        // Draw the optional label
        if let Some(value_label) = &self.value_label {
            crate::font().render(
                &format!("{value_label}: {}", self.value().round()),
                self.offset + (self.length + 12.0, 2.0),
                canvas,
            );
        }
    }

    /// Actual value of the slider.
    pub fn value(&self) -> f64 {
        (self.max - self.min) * self.pos + self.min
    }
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            offset: Vec2::zero(),
            length: 100.0,
            min: 0.0,
            max: 1.0,
            steps: None,
            pos: 0.0,
            dragged: false,
            value_label: None,
        }
    }
}
