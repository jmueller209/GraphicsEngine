use bevy_ecs::prelude::*;
use glam::{Mat4, Vec3, Quat};
use crate::ecs_components::camera::*;
use crate::ecs_components::transform::*;

#[derive(Bundle)]
pub struct FlyCameraBundle {
    pub fly_cam: FlyCamera,
    pub settings: CameraSettings,
    pub matrices: CameraMatrices,
    pub transform: Transform,
    pub marker: PrimaryCamera,
}

impl FlyCameraBundle {
    pub fn new() -> Self {
        Self {
            fly_cam: FlyCamera::default(),
            settings: CameraSettings {
                fovy: 45.0,
                znear: 0.1,
                zfar: 1000.0,
                aspect_ratio: 16.0 / 9.0,
            },
            matrices: CameraMatrices {
                view_proj: Mat4::IDENTITY,
            },
            transform: Transform {
                position: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            marker: PrimaryCamera,
        }
    }
}
