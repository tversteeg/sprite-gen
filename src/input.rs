use vek::Vec2;

/// Current input.
#[derive(Debug, Default)]
pub struct Input {
    pub mouse_pos: Vec2<i32>,

    pub left_mouse: ButtonState,
    pub right_mouse: ButtonState,
    pub up: ButtonState,
    pub down: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,
    pub space: ButtonState,

    pub r: ButtonState,
    pub g: ButtonState,
    pub c: ButtonState,
    pub o: ButtonState,
    pub n: ButtonState,
    pub x: ButtonState,
}

impl Input {
    /// Unset the released state.
    pub fn update(&mut self) {
        self.left_mouse.update();
        self.right_mouse.update();
        self.up.update();
        self.down.update();
        self.left.update();
        self.right.update();
        self.space.update();
        self.r.update();
        self.g.update();
        self.c.update();
        self.o.update();
        self.n.update();
        self.x.update();
    }
}

/// Input button state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonState {
    /// Button is not being pressed.
    #[default]
    None,
    /// Button was released this update tick.
    Released,
    /// Button is being pressed down this update tick.
    Pressed,
    /// Button is being held down.
    Down,
}

impl ButtonState {
    /// Whether the button is pressed this tick.
    pub fn is_pressed(&self) -> bool {
        *self == Self::Pressed
    }

    /// Whether the button is being held down.
    pub fn is_down(&self) -> bool {
        *self == Self::Pressed || *self == Self::Down
    }

    /// Whether the button is released this tick.
    pub fn is_released(&self) -> bool {
        *self == Self::Released
    }

    /// Move state from released to none.
    pub fn update(&mut self) {
        if *self == Self::Released {
            *self = Self::None;
        } else if *self == Self::Pressed {
            *self = Self::Down;
        }
    }

    /// Handle the window state.
    pub fn handle_bool(&mut self, pressed: bool) {
        if (*self == Self::None || *self == Self::Released) && pressed {
            *self = Self::Pressed;
        } else if (*self == Self::Pressed || *self == Self::Down) && !pressed {
            *self = Self::Released;
        }
    }
}
