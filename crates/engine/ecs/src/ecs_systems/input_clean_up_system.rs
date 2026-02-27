use bevy_ecs::prelude::*;
use crate::ecs_resources::{RawInputState, ActionState};

pub fn input_clean_up_system(
    mut raw_input: ResMut<RawInputState>,
    mut action_state: ResMut<ActionState>
) {
    raw_input.reset_mouse_delta();
    action_state.update_previous();
}
