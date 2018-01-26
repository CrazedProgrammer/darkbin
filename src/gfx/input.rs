use std::collections::HashMap;
use sdl2::rect::Point;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

#[derive(Clone)]
pub struct Input {
    keys: HashMap<Scancode, bool>,
    prev_keys: HashMap<Scancode, bool>,
    mouse_buttons: HashMap<MouseButton, bool>,
    prev_mouse_buttons: HashMap<MouseButton, bool>,
    pub mouse_pos: Point,
    pub mouse_wheel: i32,
}

impl Input {
    pub fn new() -> Input {
        Input {
            keys: HashMap::new(),
            prev_keys: HashMap::new(),
            mouse_buttons: HashMap::new(),
            prev_mouse_buttons: HashMap::new(),
            mouse_pos: Point::new(0, 0),
            mouse_wheel: 0,
        }
    }

    pub fn push_next(&mut self) {
        self.prev_keys = self.keys.clone();
        self.prev_mouse_buttons = self.mouse_buttons.clone();
        self.mouse_wheel = 0;
    }

    // getters
    pub fn get_key(&self, key: Scancode) -> bool {
        *self.keys.get(&key).unwrap_or(&false)
    }

    pub fn get_key_down(&self, key: Scancode) -> bool {
        !*self.prev_keys.get(&key).unwrap_or(&false) && *self.keys.get(&key).unwrap_or(&false)
    }

    pub fn get_key_up(&self, key: Scancode) -> bool {
        *self.prev_keys.get(&key).unwrap_or(&false) && !*self.keys.get(&key).unwrap_or(&false)
    }

    pub fn get_mouse_button(&self, mouse_button: MouseButton) -> bool {
        *self.mouse_buttons.get(&mouse_button).unwrap_or(&false)
    }

    pub fn get_mouse_button_down(&self, mouse_button: MouseButton) -> bool {
        !*self.prev_mouse_buttons.get(&mouse_button).unwrap_or(&false) && *self.mouse_buttons.get(&mouse_button).unwrap_or(&false)
    }

    pub fn get_mouse_button_up(&self, mouse_button: MouseButton) -> bool {
        *self.prev_mouse_buttons.get(&mouse_button).unwrap_or(&false) && !*self.mouse_buttons.get(&mouse_button).unwrap_or(&false)
    }

    // setters
    pub fn key_down(&mut self, key: Scancode) {
        self.keys.insert(key, true);
    }

    pub fn key_up(&mut self, key: Scancode) {
        self.keys.insert(key, false);
    }

    pub fn mouse_button_down(&mut self, mouse_button: MouseButton) {
        self.mouse_buttons.insert(mouse_button, true);
    }

    pub fn mouse_button_up(&mut self, mouse_button: MouseButton) {
        self.mouse_buttons.insert(mouse_button, false);
    }
}
