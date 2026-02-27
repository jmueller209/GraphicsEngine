use crate::Transform;
use bevy_ecs::prelude::*;
use crate::ecs_components::lights::*;
use glam::Vec3;

#[derive(Bundle)]
pub struct PointLightBundle {
    pub transform: Transform,
    pub point_light: PointLight,
}

impl PointLightBundle {
    pub fn new(position: Vec3, color: Vec3, intensity: f32, range: f32) -> Self {
        Self {
            transform: Transform {
                position,
                rotation: glam::Quat::IDENTITY,
                scale: glam::Vec3::ONE,
            },
            point_light: PointLight {
                color,
                intensity,
                range,
            },
        }
    }
}
