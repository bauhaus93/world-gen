use std::collections::BTreeMap;
use std::time::Instant;

pub struct Input {
    time_passed: u32,
    keys_pressed: BTreeMap<String, Instant>,
    mouse_delta: Option<(f64, f64)>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            time_passed: 0,
            keys_pressed: BTreeMap::new(),
            mouse_delta: None,
        }
    }
}

impl Input {
    pub fn set_time_passed(&mut self, time_passed: u32) {
        self.time_passed = time_passed;
    }

    pub fn set_key_pressed(&mut self, key: &str, pressed: bool) {
        match pressed {
            true => {
                self.keys_pressed
                    .entry(key.to_string())
                    .or_insert(Instant::now());
            }
            false => match self.keys_pressed.remove(key) {
                _ => {}
            },
        }
    }

    pub fn clear_key(&mut self, key: &str) {
        self.set_key_pressed(key, false);
    }

    pub fn set_mouse_delta(&mut self, delta: (f64, f64)) {
        self.mouse_delta = Some(delta);
    }

    pub fn clear_mouse_delta(&mut self) {
        self.mouse_delta = None;
    }

    pub fn key_pressed(&self, key: &str) -> u32 {
        match self.keys_pressed.get(key) {
            Some(key_down) => {
                (*key_down).elapsed().as_secs() as u32 * 1000
                    + (*key_down).elapsed().subsec_millis()
            }
            None => 0,
        }
    }

    pub fn has_mouse_delta(&self) -> bool {
        self.mouse_delta.is_some()
    }

    pub fn get_mouse_delta(&self) -> (f64, f64) {
        match self.mouse_delta {
            Some(d) => d,
            _ => (0., 0.),
        }
    }

    pub fn get_time_passed(&self) -> u32 {
        self.time_passed
    }

    pub fn get_movement_keys_down(&self) -> Option<[bool; 4]> {
        let mut move_down: [bool; 4] = [false, false, false, false];
        const ORDER: &'static str = "WASD";
        for i in 0..4 {
            move_down[i] = match self.keys_pressed.get(&ORDER[i..i + 1]) {
                Some(_) => true,
                None => false,
            };
        }

        if move_down.iter().any(|&e| e) {
            Some(move_down)
        } else {
            None
        }
    }
}
