use bevy_ecs::prelude::Resource;
use bytemuck::{Pod, Zeroable};
use crate::BindGroupLayout;

#[repr(C)]
#[derive(Resource, Debug, Copy, Clone, Pod, Zeroable, Default)]
pub struct CameraUniform {
    pub view_proj_matrix: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj_matrix: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}

impl BindGroupLayout for CameraUniform {
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        })
    }
}
