use bevy_ecs::prelude::*;
use glam::{Mat4};
use crate::ecs_components::{PrimaryCamera, CameraMatrices};
use crate::ecs_resources::CameraUniform;

pub fn sync_camera_uniform_system(
    query: Query<&CameraMatrices, With<PrimaryCamera>>,
    mut bridge: ResMut<CameraUniform>,
) {
    if let Ok(matrices) = query.single() {
        bridge.camera_uniform = matrices.view_proj.to_cols_array_2d();
    }
}
