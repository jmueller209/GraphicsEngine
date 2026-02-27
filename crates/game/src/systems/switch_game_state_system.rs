use bevy_ecs::prelude::*;
use engine_ecs::{ActionState, GameState};

pub fn switch_game_state_system(
    actions: Res<ActionState>,
    mut game_state: ResMut<GameState>,
) {
    match game_state.active_state.as_str() {
        "playing" => {
            if actions.just_pressed("toggle_pause") {
                game_state.set_state("paused");
                println!("Game Paused");
            }
        },
        "paused" => {
            if actions.just_pressed("toggle_pause") {
                game_state.set_state("playing");
                println!("Game Resumed");
            }
        },
        _ => {},
    }
}
