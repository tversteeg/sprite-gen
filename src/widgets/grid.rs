use sprite_gen::MaskValue;
use taffy::prelude::{Layout, Node};
use vek::{Extent2, Vec2};

use crate::{input::Input, SIZE};

/// A simple slider widget.
#[derive(Debug)]
pub struct Grid {
    /// Top-left position of the widget in pixels.
    pub offset: Vec2<f64>,
    /// Size of the grid.
    pub size: Extent2<usize>,
    /// Size of each grid item in pixels.
    pub scaling: Extent2<usize>,
    /// Each value of the grid.
    pub values: Vec<MaskValue>,
    /// Which item is hovered over by the mouse.
    pub hover_pos: Option<Vec2<usize>>,
    /// Taffy layout node.
    pub node: Node,
}

impl Grid {
    /// Construct a new grid.
    pub fn new(node: Node, size: Extent2<usize>) -> Self {
        let values = vec![MaskValue::Empty; size.w * size.h];
        let hover_pos = None;
        let scaling = Extent2::zero();
        let offset = Vec2::zero();

        Self {
            offset,
            size,
            scaling,
            values,
            hover_pos,
            node,
        }
    }

    /// Handle the input.
    ///
    /// Return when a value got updated.
    pub fn update(&mut self, input: &Input, value_on_click: MaskValue) -> bool {
        let mouse: Vec2<f64> = input.mouse_pos.as_();
        if mouse.x >= self.offset.x
            && mouse.y >= self.offset.y
            && mouse.x < self.offset.x + self.width() as f64
            && mouse.y < self.offset.y + self.height() as f64
        {
            let x = (input.mouse_pos.x as f64 - self.offset.x) / self.scaling.w as f64;
            let y = (input.mouse_pos.y as f64 - self.offset.y) / self.scaling.h as f64;
            let pos = Vec2::new(x, y).as_();
            self.hover_pos = Some(pos);

            // Handle click, returning whether a tile got changed
            let index = pos.x + pos.y * self.size.w;
            if input.left_mouse.is_down() {
                let prev = self.values[index].clone();
                self.values[index] = value_on_click.clone();

                prev != value_on_click
            } else if input.right_mouse.is_down() {
                let prev = self.values[index].clone();
                self.values[index] = MaskValue::Empty;

                prev != MaskValue::Empty
            } else {
                false
            }
        } else {
            self.hover_pos = None;

            false
        }
    }

    /// Render the slider.
    pub fn render(&self, canvas: &mut [u32]) {
        // Draw tiles
        for y in 0..self.height() {
            let start = self.offset.x as usize + (self.offset.y as usize + y) * SIZE.w;
            let y_descaled = y / self.scaling.h;

            for x in 0..self.size.w {
                let start = start + x * self.scaling.w;

                // Offset grid pattern
                let is_filled =
                    (y_descaled % 2 == 0 && x % 2 == 0) || (y_descaled % 2 == 1 && x % 2 == 1);

                // The color
                let mut color =
                    Self::mask_value_color(self.values[x + y_descaled * self.size.w].clone());

                if is_filled {
                    color ^= 0x000A0A0A;
                }

                canvas[start..(start + self.scaling.w)].fill(color);
            }
        }

        // Highlight the active tile
        if let Some(hover) = self.hover_pos {
            for y in 0..self.scaling.h {
                let start = self.offset.x as usize
                    + (hover.x * self.scaling.w)
                    + (self.offset.y as usize + y + (hover.y * self.scaling.h)) * SIZE.w;
                canvas
                    .iter_mut()
                    .skip(start)
                    .take(self.scaling.w)
                    .for_each(|pixel| *pixel ^= 0x00101010);
            }
        }
    }

    /// Resize the grid.
    pub fn resize(&mut self, size: Extent2<usize>, scaling: Extent2<usize>) {
        if self.size != size {
            self.size = size;

            self.values.resize(self.size.product(), MaskValue::Empty);
        }
        self.scaling = scaling;
    }

    /// Update from layout changes.
    pub fn update_layout(&mut self, location: Vec2<f64>, layout: &Layout) {
        self.scaling.w = layout.size.width as usize / self.size.w;
        self.scaling.h = layout.size.width as usize / self.size.h;

        self.offset = location;
    }

    /// Clear the grid.
    pub fn clear(&mut self) {
        self.values.fill(MaskValue::Empty);
    }

    /// Get the resulting mask.
    pub fn mask(&self) -> &[MaskValue] {
        &self.values
    }

    /// Total width.
    pub fn width(&self) -> usize {
        self.size.w * self.scaling.w
    }

    /// Total height.
    pub fn height(&self) -> usize {
        self.size.h * self.scaling.h
    }

    /// Color for a maskvalue.
    fn mask_value_color(mask_value: MaskValue) -> u32 {
        match mask_value {
            MaskValue::Solid => 0xFF444444,
            MaskValue::Empty => 0xFFFFFFFF,
            MaskValue::Body1 => 0xFFFF9999,
            MaskValue::Body2 => 0xFF9999FF,
        }
    }
}
