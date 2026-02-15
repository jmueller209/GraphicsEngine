use bevy_ecs::prelude::*;
use glam::{Mat4, Vec3};

#[derive(Component)]
pub struct CameraSettings {
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub aspect_ratio: f32,
}

#[derive(Component)]
pub struct CameraMatrices {
    pub view_proj: Mat4,
}

#[derive(Component)]
pub struct PrimaryCamera;

#[derive(Component)]
pub struct FlyCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            speed: 10.0,
            sensitivity: 0.002,
        }
    }
}

#[derive(Component)]
pub struct TargetCamera {
    pub target_entity: Entity,
    pub offset: Vec3,
}
