use bevy_ecs::prelude::*;
use glam::{Mat4};
use crate::ecs_components::{Transform, CameraSettings, CameraMatrices};

pub fn camera_matrix_system(
    mut query: Query<(&Transform, &CameraSettings, &mut CameraMatrices)>
) {
    puffin::profile_function!();
    for (transform, settings, mut matrices) in &mut query {

        let forward = transform.rotation * glam::Vec3::NEG_Z;
        let up = transform.rotation * glam::Vec3::Y;
        
        let view = Mat4::look_to_rh(
            transform.position,
            forward,
            up
        );

        let proj = Mat4::perspective_rh(
            settings.fovy.to_radians(),
            settings.aspect_ratio,
            settings.znear,
            settings.zfar,
        );

        matrices.view_proj = proj * view;
    }
}
