use bevy_ecs::prelude::*;
use crate::ecs_resources::input::{ActionState, RawInputState, InputBindings};
use crate::ecs_resources::game_state::GameState; // Deine neue Container-Resource

pub fn input_mapping_system(
    game_state: Res<GameState>,
    raw: Res<RawInputState>,
    bindings: Res<InputBindings>,
    mut action_state: ResMut<ActionState>,
) {
    action_state.active_actions.clear();

    let context_name = &game_state.active_state;

    if let Some(context_map) = bindings.bindings.get(context_name) {
        for (action_name, key_code) in context_map {
            if raw.pressed_keys.contains(key_code) {
                action_state.active_actions.insert(action_name.clone());
            }
        }
    }
}
