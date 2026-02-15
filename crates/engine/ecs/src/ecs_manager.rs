use bevy_ecs::prelude::*;
use crate::ecs_resources::frame_context::FrameContext;

pub struct ECSManager {
    pub world: World,
    pub schedule: Schedule,
}

#[derive(Resource, Default)]
pub struct GameTime {
    pub delta_seconds: f32,
}

impl ECSManager {
    pub fn new() -> Self {
        let mut world = World::new();
        let schedule = Schedule::default();

        world.insert_resource(GameTime::default());

        Self { world, schedule }
    }

    pub fn update(&mut self, ctx: FrameContext) {
        self.world.insert_resource(ctx);
        self.schedule.run(&mut self.world);
    }
}
