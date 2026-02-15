use bevy_ecs::prelude::Resource;

#[derive(Resource, Default)]
pub struct CameraUniform {
    pub camera_uniform: [[f32; 4]; 4],
}
