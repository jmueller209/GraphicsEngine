use bevy_ecs::prelude::*;
use crate::ecs_components::{Transform, FlyCamera};
use crate::ecs_resources::input::{ActionState, RawInputState};
use crate::ecs_resources::frame_context::FrameContext;
use glam::{Vec3, Quat, EulerRot};

pub fn fly_camera_controller_system(
    ctx: Res<FrameContext>,
    actions: Res<ActionState>,
    raw: Res<RawInputState>,
    mut query: Query<&mut Transform, With<FlyCamera>>,
) {
    let move_speed = 10.0;
    let look_sensitivity = 0.002;
    let dt = ctx.dt;

    for mut transform in &mut query {
        if raw.mouse_delta.length_squared() > 0.001 {
            let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            yaw -= raw.mouse_delta.x * look_sensitivity;
            pitch -= raw.mouse_delta.y * look_sensitivity;
            pitch = pitch.clamp(-1.54, 1.54);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
        }

        let mut velocity = Vec3::ZERO;
        
        let forward = transform.rotation * Vec3::NEG_Z;
        let right = transform.rotation * Vec3::X;
        let up = Vec3::Y; 

        if actions.active_actions.contains("move_forward") { velocity += forward; }
        if actions.active_actions.contains("move_backward") { velocity -= forward; }
        if actions.active_actions.contains("move_left") { velocity -= right; }
        if actions.active_actions.contains("move_right") { velocity += right; }
        if actions.active_actions.contains("move_up") { velocity += up; }
        if actions.active_actions.contains("move_down") { velocity -= up; }

        if velocity != Vec3::ZERO {
            transform.position += velocity.normalize() * move_speed * dt;
        }
    }
}
