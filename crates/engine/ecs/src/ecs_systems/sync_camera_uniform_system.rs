use bevy_ecs::prelude::*;
use crate::ecs_components::{PrimaryCamera, CameraMatrices};
use engine_gpu_types::CameraUniform;

pub fn sync_camera_uniform_system(
    query: Query<&CameraMatrices, With<PrimaryCamera>>,
    mut bridge: ResMut<CameraUniform>,
) {
    puffin::profile_function!();
    if let Ok(matrices) = query.single() {
        bridge.view_proj_matrix = matrices.view_proj.to_cols_array_2d();
    }
}
