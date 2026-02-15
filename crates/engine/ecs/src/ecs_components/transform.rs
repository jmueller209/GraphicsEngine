use bevy_ecs::prelude::*;

// Transformation Component: position, rotation, scale
#[derive(Component, Debug, Clone, Copy)] // Copy is nice for math structs
pub struct Transform {
    pub position: glam::Vec3,
    pub rotation: glam::Quat,
    pub scale: glam::Vec3, // Scale is rarely used but essential for the Matrix
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            position: glam::Vec3::ZERO,
            rotation: glam::Quat::IDENTITY,
            scale: glam::Vec3::ONE,
        }
    }

    pub fn to_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_scale_rotation_translation(
            self.scale, 
            self.rotation, 
            self.position
        )
    }
}
