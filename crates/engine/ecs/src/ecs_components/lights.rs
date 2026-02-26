use bevy_ecs::prelude::Component;
use glam::Vec3;

#[derive(Component, Debug, Clone, Copy)]
pub struct PointLight {
    pub color: Vec3,
    pub intensity: f32,
    pub range: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct SpotLight {
    pub color: Vec3,
    pub intensity: f32,
    pub range: f32,
    pub direction: Vec3,
    pub cutoff_angle: f32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct DirectionalLight {
    pub color: Vec3,
    pub intensity: f32,
    pub direction: Vec3,
}
