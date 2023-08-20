use std::collections::HashMap;
pub use winit::event::{ElementState, VirtualKeyCode, ButtonId};

/// Keymap is used to store key states from winit.
/// Keymap maps keycodes to (current state, previous state) to allow for detection of change state.
#[derive(Debug)]
pub struct Input {
    pub keys: KeyMap,
    pub buttons: ButtonMap,
    pub scroll: MouseScroll,
    pub mouse: MouseMove,
}
impl Input {
    pub fn new() -> Self {
        Self {
            keys: KeyMap::new(),
            buttons: ButtonMap::new(),
            scroll: MouseScroll::new(),
            mouse: MouseMove::new(),
        }

    }
}

#[derive(Debug)]
pub struct KeyMap(pub HashMap<VirtualKeyCode, (ElementState, ElementState)>);
#[derive(Debug)]
pub struct ButtonMap(pub HashMap<ButtonId, (ElementState, ElementState)>);
#[derive(Debug)]
pub struct MouseScroll {
    x: f32,
    y: f32,
}

#[derive(Debug)]
pub struct MouseMove {
    x: f32,
    y: f32,
}

impl MouseMove{
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
    pub fn set(&mut self, x: f32, y:f32) {
        self.x = x;
        self.y = y;
    }
    pub fn delta(&mut self) -> (f32, f32) {
        let (x, y) = (self.x, self.y);
        self.x = 0.0;
        self.y = 0.0;
        (x, y)
    }
}
impl MouseScroll{
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
    pub fn set(&mut self, x: f32, y:f32) {
        self.x = x;
        self.y = y;
    }
    pub fn delta(&mut self) -> (f32, f32) {
        let (x, y) = (self.x, self.y);
        self.x = 0.0;
        self.y = 0.0;
        (x, y)
    }
}


impl KeyMap {
    /// Returns a new empty hashmap
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn pressed(&mut self, keycode: VirtualKeyCode) -> bool {
        if let Some((current_state, previous_state)) = self.0.get(&keycode) {
            if *current_state == ElementState::Pressed && *previous_state == ElementState::Released
            {
                self.0.insert(keycode, (ElementState::Pressed, ElementState::Pressed));
                return true;
            }
        }
        false
    }
    pub fn held(&self, keycode: VirtualKeyCode) -> bool {
        if let Some((current_state, _)) = self.0.get(&keycode) {
            if *current_state == ElementState::Pressed
            {
                return true;
            }
        }
        false
    }
}

impl ButtonMap {
    /// Returns a new empty hashmap
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn pressed(&mut self, button: ButtonId) -> bool {
        if let Some((current_state, previous_state)) = self.0.get(&button) {
            if *current_state == ElementState::Pressed && *previous_state == ElementState::Released
            {
                self.0.insert(button, (ElementState::Pressed, ElementState::Pressed));
                return true;
            }
        }
        false
    }
    pub fn held(&self, button: ButtonId) -> bool {
        if let Some((current_state, _)) = self.0.get(&button) {
            if *current_state == ElementState::Pressed
            {
                return true;
            }
        }
        false
    }
}
