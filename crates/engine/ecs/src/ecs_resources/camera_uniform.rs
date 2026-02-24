use bevy_ecs::prelude::Resource;

#[repr(C)]
#[derive(Resource, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub camera_uniform: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            camera_uniform: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}


