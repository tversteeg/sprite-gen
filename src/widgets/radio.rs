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
}

impl<const N: usize> Radio<N> {
    /// Construct a new checkbox button.
    pub fn new(
        offset: Vec2<f64>,
        boxes: [&str; N],
        title: Option<String>,
        selected: usize,
    ) -> Self {
        assert!(selected < N);

        let mut index = 0;
        let boxes = boxes.map(|label| {
            let checkbox = Checkbox::new(
                // Move each consecutive radio checkbox down a set amount of pixels, also keeping space for the title if applicable
                offset
                    + (
                        0.0,
                        index as f64 * 30.0 + if title.is_some() { 20.0 } else { 0.0 },
                    ),
                Some(label.to_string()),
                index == selected,
            );
            index += 1;

            checkbox
        });

        Self {
            offset,
            title,
            selected,
            boxes,
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
}
