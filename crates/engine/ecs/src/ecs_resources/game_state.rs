use bevy_ecs::prelude::*;
use std::collections::HashMap;

pub struct GameStateConfig {
    pub name: &'static str,
    pub cursor_visible: bool,
}

#[derive(Resource)]
pub struct GameState {
    pub active_state: String,
    cursor_lookup: HashMap<String, bool>,
}

impl Default for GameState {
    fn default() -> Self {
        let mut cursor_lookup = HashMap::new();
        cursor_lookup.insert("playing".to_string(), false);
        Self {
            active_state: "playing".to_string(),
            cursor_lookup,
        }
    }
}

impl GameState {
    pub fn set_config(&mut self, configs: &[GameStateConfig], initial_state: &str) {
        let mut new_lookup = HashMap::new();
        for config in configs {
            new_lookup.insert(config.name.to_string(), config.cursor_visible);
        }
        if new_lookup.contains_key(initial_state) {
            self.cursor_lookup = new_lookup;
            self.active_state = initial_state.to_string();
        } else {
            eprintln!("Warnung: Neuer Initial-State '{}' nicht in Config gefunden!", initial_state);
        }
    }

    pub fn is_cursor_visible(&self) -> bool {
        *self.cursor_lookup.get(&self.active_state).unwrap_or(&true)
    }

    pub fn set_state(&mut self, new_state: &str) {
        if self.cursor_lookup.contains_key(new_state) {
            self.active_state = new_state.to_string();
        }
    }
}


