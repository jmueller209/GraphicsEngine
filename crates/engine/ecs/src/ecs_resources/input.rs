use bevy_ecs::prelude::*;
use std::collections::{HashMap, HashSet};
use winit::keyboard::KeyCode;
use serde::Deserialize;
use glam::Vec2;

#[derive(Deserialize, Debug)]
struct JsonConfig(HashMap<String, HashMap<String, String>>);

#[derive(Resource, Debug)]
pub struct InputBindings {
    pub bindings: HashMap<String, HashMap<String, KeyCode>>,
}

impl InputBindings {
    pub fn load_from_file(&mut self, path: &str) {
        let file_content = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Could not find path {}", path));

        let config: JsonConfig = serde_json::from_str(&file_content)
            .expect("Error parsing JSON");

        self.bindings.clear();

        for (context, actions) in config.0 {
            let mut context_map = HashMap::new();
            for (action_name, key_string) in actions {
                if let Some(key_code) = self.string_to_keycode(&key_string) {
                    context_map.insert(action_name, key_code);
                } else {
                    eprintln!("Warning: Unknown key {} in action {}.", key_string, action_name);
                }
            }
            self.bindings.insert(context, context_map);
        }
    }

    fn string_to_keycode(&self, key_str: &str) -> Option<KeyCode> {
        let json_string = format!("\"{}\"", key_str);
        serde_json::from_str::<KeyCode>(&json_string).ok()
    }
}

impl Default for InputBindings {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        let mut playing_map = HashMap::new();
        playing_map.insert("move_forward".to_string(), KeyCode::KeyW);
        playing_map.insert("move_backward".to_string(), KeyCode::KeyS);
        playing_map.insert("move_left".to_string(), KeyCode::KeyA);
        playing_map.insert("move_right".to_string(), KeyCode::KeyD);
        playing_map.insert("move_up".to_string(), KeyCode::Space);
        playing_map.insert("move_down".to_string(), KeyCode::ShiftLeft);
        playing_map.insert("toggle_pause".to_string(), KeyCode::Escape);
        playing_map.insert("interact".to_string(), KeyCode::KeyE);
        bindings.insert("playing".to_string(), playing_map);
        Self { bindings }
    }
}

#[derive(Resource, Default, Debug)]
pub struct ActionState {
    pub active_actions: HashSet<String>,
    pub previous_actions: HashSet<String>,
}

impl ActionState {
    pub fn is_pressed(&self, action: &str) -> bool {
        self.active_actions.contains(action)
    }
    pub fn just_pressed(&self, action: &str) -> bool {
        self.active_actions.contains(action) && !self.previous_actions.contains(action)
    }
    pub fn update_previous(&mut self) {
        self.previous_actions = self.active_actions.clone();
    }
}

#[derive(Resource, Default, Debug)]
pub struct RawInputState {
    pub pressed_keys: HashSet<KeyCode>,
    pub mouse_delta: Vec2,
    pub mouse_buttons: HashSet<winit::event::MouseButton>,
}

impl RawInputState {
    pub fn reset_mouse_delta(&mut self) {
        self.mouse_delta = Vec2::ZERO;
    }
}
