use bevy_ecs::prelude::*;
use engine_gpu_types::{LightInstanceUniform, GlobalLightDataUniform};
use crate::ecs_components::{Transform, PointLight, SpotLight, DirectionalLight};

pub fn sync_lights_uniform_system(
    query_point: Query<(&PointLight, &Transform)>,
    query_spot: Query<(&SpotLight, &Transform)>,
    query_dir: Query<&DirectionalLight>,
    mut light_data: ResMut<GlobalLightDataUniform>,
) {
    puffin::profile_function!();
    if let Some(dir_light) = query_dir.iter().next() {
        light_data.sun_direction = [
            dir_light.direction.x,
            dir_light.direction.y,
            dir_light.direction.z,
            0.0,
        ];
        light_data.sun_color = [
            dir_light.color.x,
            dir_light.color.y,
            dir_light.color.z,
            dir_light.intensity,
        ];
    }

    let mut light_count = 0;

    for (light, transform) in query_point.iter() {
        if light_count >= 16 { break; }
        
        light_data.lights[light_count] = LightInstanceUniform {
            position: transform.position.to_array(),
            light_type: 0, // 0 for PointLight
            color: light.color.to_array(),
            intensity: light.intensity,
            range: light.range,
            direction: [0.0; 3], // Irrelevant
            cutoff: 0.0,        // Irrelevant 
            _padding: [0.0; 3],
        };
        light_count += 1;
    }

    for (light, transform) in query_spot.iter() {
        if light_count >= 16 { break; }

        light_data.lights[light_count] = LightInstanceUniform {
            position: transform.position.to_array(),
            light_type: 1, // 1 for SpotLight
            color: light.color.to_array(),
            intensity: light.intensity,
            direction: light.direction.to_array(),
            range: light.range,
            cutoff: light.cutoff_angle.cos(), 
            _padding: [0.0; 3],
        };
        light_count += 1;
    }

    light_data.num_lights = light_count as u32;
}

