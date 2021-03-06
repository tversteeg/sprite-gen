use druid::kurbo::*;
use druid::piet::*;
use druid::*;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use sprite_gen::{MaskValue, Options};
use std::sync::RwLock;

pub const MAX_GRID_SIZE: usize = 128;
pub const MAX_RESULTS: usize = 1000;

lazy_static! {
    pub static ref GRID: RwLock<Vec<MaskValue>> =
        RwLock::new(vec![MaskValue::Empty; MAX_GRID_SIZE * MAX_GRID_SIZE]);
    pub static ref RESULTS: RwLock<Vec<(usize, usize, Vec<u8>)>> = RwLock::new(Vec::new());
}

#[derive(Debug, Clone, PartialEq, Data, Lens, Serialize, Deserialize)]
pub struct AppState {
    pub results_amount: f64,
    pub fill_type: i8,
    pub size_x: f64,
    pub size_y: f64,
    pub render_scale: f64,
    pub edge_brightness: f64,
    pub color_variations: f64,
    pub brightness_noise: f64,
    pub saturation: f64,
    pub mirror_x: bool,
    pub mirror_y: bool,
    pub colored: bool,
    pub file_path: Option<String>,
}

impl AppState {
    // Size of each grid block
    pub fn block_size(&self, total_area: &Size) -> f64 {
        (total_area.width / self.width() as f64).min(total_area.height / self.height() as f64)
    }

    pub fn results(&self) -> usize {
        (self.results_amount * MAX_RESULTS as f64).floor().max(1.0) as usize
    }

    pub fn width(&self) -> usize {
        (self.size_x * MAX_GRID_SIZE as f64).floor().max(1.0) as usize
    }

    pub fn result_width(&self) -> usize {
        if self.mirror_x {
            self.width() * 2
        } else {
            self.width()
        }
    }

    pub fn result_height(&self) -> usize {
        if self.mirror_y {
            self.height() * 2
        } else {
            self.height()
        }
    }

    pub fn height(&self) -> usize {
        (self.size_y * MAX_GRID_SIZE as f64).floor().max(1.0) as usize
    }

    pub fn options(&self) -> Options {
        Options {
            mirror_x: self.mirror_x,
            mirror_y: self.mirror_y,
            edge_brightness: self.edge_brightness as f32,
            color_variations: self.color_variations as f32 + 0.01,
            brightness_noise: self.brightness_noise as f32 + 0.01,
            saturation: self.saturation as f32,
            colored: self.colored,
            seed: 0,
        }
    }

    pub fn pixels(&self) -> Vec<MaskValue> {
        let width = self.width();
        let height = self.height();
        GRID.read()
            .unwrap()
            .iter()
            // Only take the size needed instead of the full 1024 * 1024
            .enumerate()
            .filter_map(move |(index, p)| {
                if index % MAX_GRID_SIZE < width && index / MAX_GRID_SIZE < height {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect::<_>()
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            results_amount: 0.1,
            size_x: 0.105,
            size_y: 0.08,
            render_scale: 6.0,
            edge_brightness: 0.3,
            color_variations: 0.2,
            brightness_noise: 0.3,
            saturation: 0.5,
            colored: true,
            mirror_x: true,
            mirror_y: false,
            file_path: None,
            fill_type: MaskValue::Solid.i8(),
        }
    }
}

pub trait MaskValueEx {
    fn color(&self) -> Color;
}

impl MaskValueEx for MaskValue {
    fn color(&self) -> Color {
        match self {
            MaskValue::Empty => Color::WHITE,
            MaskValue::Solid => Color::grey8(64),
            MaskValue::Body1 => Color::rgb8(255, 128, 128),
            MaskValue::Body2 => Color::rgb8(128, 128, 255),
        }
    }
}
