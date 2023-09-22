use blit::BlitOptions;
use vek::{Extent2, Rect, Vec2};

use crate::input::Input;

/// A simple button widget.
#[derive(Debug)]
pub struct Button {
    /// Top-left position of the widget in pixels.
    pub offset: Vec2<f64>,
    /// Size of the button in pixels.
    pub size: Extent2<f64>,
    /// Extra size of the click region in pixels.
    ///
    /// Relative to the offset.
    pub click_region: Option<Rect<f64, f64>>,
    /// A custom label with text centered at the button.
    pub label: Option<String>,
    /// Current button state.
    pub state: State,
}

impl Button {
    /// Handle the input.
    ///
    /// Return when the button is released.
    pub fn update(&mut self, input: &Input) -> bool {
        let mut rect = Rect::new(self.offset.x, self.offset.y, self.size.w, self.size.h);
        if let Some(mut click_region) = self.click_region {
            click_region.x += self.offset.x;
            click_region.y += self.offset.y;
            rect = rect.union(click_region);
        }

        match self.state {
            State::Normal => {
                if !input.left_mouse.is_down() && rect.contains_point(input.mouse_pos.as_()) {
                    self.state = State::Hover;
                }

                false
            }
            State::Hover => {
                if !rect.contains_point(input.mouse_pos.as_()) {
                    self.state = State::Normal;
                } else if input.left_mouse.is_down() {
                    self.state = State::Down;
                }

                false
            }
            State::Down => {
                if input.left_mouse.is_released() {
                    self.state = State::Normal;
                    true
                } else {
                    false
                }
            }
        }
    }

    /// Render the slider.
    pub fn render(&self, canvas: &mut [u32]) {
        let button = crate::sprite(match self.state {
            State::Normal => "button-normal",
            State::Hover => "button-hover",
            State::Down => "button-down",
        });
        button.render_options(
            canvas,
            &BlitOptions::new_position(self.offset.x, self.offset.y)
                .with_slice9((2, 2, 1, 2))
                .with_area((self.size.w, self.size.h)),
        );

        if let Some(label) = &self.label {
            crate::font().render_centered(
                label,
                self.offset + (self.size.w / 2.0, self.size.h / 2.0),
                canvas,
            );
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            offset: Vec2::zero(),
            size: Extent2::zero(),
            label: None,
            state: State::default(),
            click_region: None,
        }
    }
}

/// In which state the button can be.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Button is doing nothing.
    #[default]
    Normal,
    /// Button is hovered over by the mouse.
    Hover,
    /// Button is hold down.
    Down,
}
