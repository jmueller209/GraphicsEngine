use bevy_ecs::prelude::*;
use std::collections::{HashMap, HashSet};
use winit::keyboard::KeyCode;
use serde::Deserialize;
use glam::Vec2;

// This resource holds the mapping from action names to key codes for different game contexts
// (e.g., "playing", "menu")
#[derive(Resource, Default, Debug)]
pub struct InputBindings {
    // context (e.g. "playing") -> map(action name -> key)
    pub bindings: HashMap<String, HashMap<String, KeyCode>>,
}

// This resource holds the current state of actions, i.e., which actions are active in the current context.
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

// This recourse holds the raw input state, i.e., which keys are currently pressed. This is used to
// determine the active actions based on the input bindings.

#[derive(Resource, Default, Debug)]
pub struct RawInputState {
    pub pressed_keys: HashSet<KeyCode>,
    pub mouse_delta: Vec2,
    pub mouse_buttons: HashSet<winit::event::MouseButton>,
}

impl RawInputState {
    /// Important: This function must be called at the end of EVERY frame to prevent mouse movement
    /// from accumulating.
    pub fn reset_mouse(&mut self) {
        self.mouse_delta = Vec2::ZERO;
    }
}


// Function for loading input bindings from a JSON file. The JSON should have the following
// structure:
// {
//   "playing": {
//     "jump": "Space",
//     "move_left": "A",
//     "move_right": "D"
//     ...
//   },
//   "menu": {
//     "select": "Enter",
//     "back": "Escape"
//     ...
//   }
//   others contexts...
// }

#[derive(Deserialize, Debug)]
struct JsonConfig(HashMap<String, HashMap<String, String>>);

pub fn load_input_bindings(path: &str) -> InputBindings {
    let mut input_bindings = InputBindings::default();

    let file_content = std::fs::read_to_string(path)
        .expect("Konnte Key-Bindings Datei nicht finden");

    let config: JsonConfig = serde_json::from_str(&file_content)
        .expect("Fehler beim Parsen der JSON");

    for (context, actions) in config.0 {
        let mut context_map = HashMap::new();
        
        for (action_name, key_string) in actions {
            if let Some(key_code) = string_to_keycode(&key_string) {
                context_map.insert(action_name, key_code);
            } else {
                eprintln!("Warnung: Unbekannter Key '{}' in Aktion '{}'", key_string, action_name);
            }
        }
        
        input_bindings.bindings.insert(context, context_map);
    }

    input_bindings
}

fn string_to_keycode(key_str: &str) -> Option<KeyCode> {
    let json_string = format!("\"{}\"", key_str);
    serde_json::from_str::<KeyCode>(&json_string).ok()
}
