use taffy::prelude::Node;
use vek::Vec2;

use crate::input::Input;

use super::checkbox::Checkbox;

/// A simple button widget.
#[derive(Debug)]
pub struct Radio<const N: usize> {
    /// Top-left position of the widget in pixels.
    pub offset: Vec2<f64>,
    /// A custom label with text.
    pub title: Option<String>,
    /// Which box is selected.
    pub selected: usize,
    /// All checkboxes.
    pub boxes: [Checkbox; N],
    /// Taffy layout node.
    pub node: Node,
}

impl<const N: usize> Radio<N> {
    /// Construct a new checkbox button.
    pub fn new(boxes: [&str; N], title: Option<String>, selected: usize, node: Node) -> Self {
        assert!(selected < N);

        let offset = Vec2::zero();
        let mut index = 0;
        let boxes = boxes.map(|label| {
            index += 1;
            Checkbox::new(Vec2::zero(), Some(label.to_string()), index - 1 == selected)
        });

        Self {
            offset,
            title,
            selected,
            boxes,
            node,
        }
    }

    /// Handle the input.
    ///
    /// Return when the a new selection is made.
    pub fn update(&mut self, input: &Input) -> Option<usize> {
        let mut changed = false;

        // Update the state of all checkboxes
        for index in 0..N {
            if self.boxes[index].update(input) {
                if self.selected != index {
                    self.selected = index;
                    changed = true;
                }

                // Unset all other checkboxes and set this one
                for index in 0..N {
                    self.boxes[index].set(index == self.selected);
                }
            }
        }

        if changed {
            Some(self.selected)
        } else {
            None
        }
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
