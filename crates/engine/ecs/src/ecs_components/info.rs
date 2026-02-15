use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct ModelId(pub usize); // Maps to your loaded WGPU models

#[derive(Component)]
pub struct Name(pub String); // For debugging and identification
