use engine_assets::data_structures::{MaterialId, MeshId};
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct MeshHandle(pub MeshId);

#[derive(Component, Debug, Clone, Copy)]
pub struct MaterialHandle(pub MaterialId);

pub struct Invisible;

