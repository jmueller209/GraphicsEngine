use bytemuck::{Pod, Zeroable};
use crate::BindGroupLayout;
use bevy_ecs::prelude::*;

#[repr(C)]
#[derive(Resource, Debug, Copy, Clone, Pod, Zeroable)]
pub struct LightInstanceUniform {
    pub position: [f32; 3],
    pub light_type: u32,
    pub color: [f32; 3],
    pub intensity: f32,
    pub direction: [f32; 3],
    pub range: f32,
    pub cutoff: f32,
    pub _padding: [f32; 3],
}

impl LightInstanceUniform {
    pub fn zeroed() -> Self {
        bytemuck::Zeroable::zeroed()
    }
}

#[repr(C)]
#[derive(Resource, Debug, Copy, Clone, Pod, Zeroable)]
pub struct GlobalLightDataUniform {
    pub ambient_color: [f32; 4],
    pub sun_direction: [f32; 4],
    pub sun_color: [f32; 4],
    pub num_lights: u32,
    pub _padding: [u32; 3],
    pub lights: [LightInstanceUniform; 16],
}

impl Default for GlobalLightDataUniform {
    fn default() -> Self {
        Self {
            ambient_color: [1.0, 1.0, 1.0, 1.0],
            sun_direction: [0.0, -1.0, 0.0, 0.0],
            sun_color: [0.0, 0.0, 0.0, 0.0],
            num_lights: 0,
            _padding: [0; 3],
            lights: [LightInstanceUniform::zeroed(); 16],
        }
    }
}

impl BindGroupLayout for GlobalLightDataUniform {
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("light_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }
}
