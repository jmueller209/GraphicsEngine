use bevy_ecs::prelude::Resource;

#[derive(Resource, Debug, Clone, Copy)]
pub struct FrameContext {
    pub dt: f32,
    pub tick: u64,
    pub total_time: f64,
}
