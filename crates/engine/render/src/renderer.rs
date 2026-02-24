use bevy_ecs::prelude::*;
use engine_assets::AssetManager;
use engine_ecs::{MeshHandle, MaterialHandle, Transform};
use wgpu::util::DeviceExt;
use engine_ecs::CameraUniform;

pub struct Renderer{
    camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,
}

impl Renderer {
    pub fn new(device: &wgpu::Device) -> Self {
        let camera_bind_group_layout = Self::create_camera_layout(device);
        let (camera_buffer, camera_bind_group) = Self::create_camera_resources(device, &camera_bind_group_layout);
        Self {
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,
        }

    }

    pub fn update_camera(&self, queue: &wgpu::Queue, camera_uniform: &CameraUniform) {
        queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[*camera_uniform]));
    }

    pub fn draw_world<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        world: &mut World,
        asset_manager: &'a AssetManager,
    ) {
        let mut query = world.query::<(&Transform, &MeshHandle, &MaterialHandle)>();

        for (transform, mesh_handle, mat_handle) in query.iter(world) {
            let mesh = asset_manager.get_mesh(mesh_handle.0);
            let material = asset_manager.get_material(mat_handle.0);
            let pipeline = asset_manager.pipeline_cache.get(&material.pipeline_name)
                .expect("Pipeline not found in cache");

            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &material.bind_group, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
        }
    }

    fn create_camera_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
    }

    /// Erstellt den tatsächlichen Speicher (Buffer) und die Verknüpfung (BindGroup)
    fn create_camera_resources(
        device: &wgpu::Device, 
        layout: &wgpu::BindGroupLayout
    ) -> (wgpu::Buffer, wgpu::BindGroup) {
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        });

        (camera_buffer, camera_bind_group)
    }


}
