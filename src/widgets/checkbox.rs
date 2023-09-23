use taffy::prelude::Node;
use vek::{Extent2, Rect, Vec2};

use crate::input::Input;

use super::button::Button;

/// A simple button widget.
#[derive(Debug)]
pub struct Checkbox {
    /// Top-left position of the widget in pixels.
    pub offset: Vec2<f64>,
    /// A custom label with text at the right of the checkbox.
    pub label: Option<String>,
    /// Checked or not.
    pub checked: bool,
    /// Checkbox button frame.
    pub button: Button,
}

impl Checkbox {
    /// Construct a new checkbox button.
    pub fn new(offset: Vec2<f64>, label: Option<String>, checked: bool) -> Self {
        let mut button = Button {
            offset,
            size: Extent2::new(20.0, 20.0),
            ..Default::default()
        };

        // Allow the checkbox to be selected by clicking the label to
        if let Some(label) = &label {
            let char_size = crate::font().char_size;
            button.click_region = Some(Rect::new(
                20.0,
                0.0,
                char_size.w as f64 * label.len() as f64 + 5.0,
                20.0,
            ));
        }

        Self {
            offset,
            checked,
            button,
            label,
        }
    }

    /// Handle the input.
    ///
    /// Return when the button is changed.
    pub fn update(&mut self, input: &Input) -> bool {
        if self.button.update(input) {
            self.checked = !self.checked;

            true
        } else {
            false
        }
    }

    /// Render the slider.
    pub fn render(&self, canvas: &mut [u32]) {
        self.button.render(canvas);

        if self.checked {
            crate::sprite("checkmark").render(canvas, self.offset + (2.0, 3.0));
        }

        if let Some(label) = &self.label {
            crate::font().render(label, self.offset + (25.0, 5.0), canvas);
        }
    }

    /// Set whether the checkbox is checked or not.
    pub fn set(&mut self, state: bool) {
        self.checked = state;
    }

    /// Update the layout.
    pub fn update_layout(&mut self, location: Vec2<f64>) {
        self.offset = location;
        self.button.offset = location;
    }
}

/// A group of checkboxes.
#[derive(Debug)]
pub struct CheckboxGroup<const N: usize> {
    /// Top-left position of the widget in pixels.
    pub offset: Vec2<f64>,
    /// A custom label with text.
    pub title: Option<String>,
    /// All checkboxes.
    pub boxes: [Checkbox; N],
    /// Taffy layout node.
    pub node: Node,
}

impl<const N: usize> CheckboxGroup<N> {
    /// Construct a new checkbox button group.
    pub fn new(boxes: [(&str, bool); N], title: Option<String>, node: Node) -> Self {
        let offset = Vec2::zero();
        let boxes = boxes
            .map(|(label, checked)| Checkbox::new(Vec2::zero(), Some(label.to_string()), checked));

        Self {
            offset,
            title,
            boxes,
            node,
        }
    }

    /// Handle the input.
    ///
    /// Return which checkbox changed.
    pub fn update(&mut self, input: &Input) -> Option<usize> {
        let mut changed = None;

        // Update the state of all checkboxes
        for index in 0..N {
            if self.boxes[index].update(input) {
                changed = Some(index);
            }
        }

        changed
    }

    /// Render the slider.
    pub fn render(&self, canvas: &mut [u32]) {
        for index in 0..N {
            self.boxes[index].render(canvas);
        }

        if let Some(label) = &self.title {
            crate::font().render(label, self.offset, canvas);
        }
    }

    /// Get the value of the checkbox at the index.
    pub fn checked(&self, index: usize) -> bool {
        assert!(index < N);

        self.boxes[index].checked
    }

    /// Update the layout.
    pub fn update_layout(&mut self, location: Vec2<f64>) {
        self.offset = location;
        for index in 0..self.boxes.len() {
            self.boxes.get_mut(index).unwrap().update_layout(
                location
                    + (
                        0.0,
                        index as f64 * 30.0 + if self.title.is_some() { 20.0 } else { 0.0 },
                    ),
            );
        }
    }
}
