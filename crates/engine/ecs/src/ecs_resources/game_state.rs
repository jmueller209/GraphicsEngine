use bevy_ecs::prelude::*;
use std::collections::HashMap;

pub struct StateDefinition {
    pub name: &'static str,
    pub cursor_visible: bool,
}

#[derive(Resource)]
pub struct GameState {
    pub active_state: String,
    cursor_lookup: HashMap<String, bool>,
}

impl GameState {
    pub fn new(configs: &[StateDefinition], initial_state: &str) -> Self {
        let mut cursor_lookup = HashMap::new();
        
        for config in configs {
            cursor_lookup.insert(config.name.to_string(), config.cursor_visible);
        }

        Self {
            active_state: initial_state.to_string(),
            cursor_lookup,
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
