use bevy_ecs::prelude::*;
use engine_assets::AssetManager;
use engine_ecs::{MeshHandle, MaterialHandle, Transform};
use engine_gpu_types::{CameraUniform, GlobalLightDataUniform, BindGroupLayout};

pub struct Renderer{
    camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub camera_bind_group_layout: wgpu::BindGroupLayout,

    light_buffer: wgpu::Buffer,
    light_bind_group: wgpu::BindGroup,
    light_bind_group_layout: wgpu::BindGroupLayout,
}

impl Renderer {
    pub fn new(device: &wgpu::Device) -> Self {
        let camera_bind_group_layout = CameraUniform::bind_group_layout(device);
        let (camera_buffer, camera_bind_group) = Self::create_uniform_resource::<CameraUniform>(device, &camera_bind_group_layout, "Camera");

        let light_bind_group_layout = GlobalLightDataUniform::bind_group_layout(device);
        let (light_buffer, light_bind_group) = Self::create_uniform_resource::<GlobalLightDataUniform>(device, &light_bind_group_layout, "Light");
        Self {
            camera_buffer,
            camera_bind_group,
            camera_bind_group_layout,

            light_buffer,
            light_bind_group,
            light_bind_group_layout,
        }

    }

    pub fn update_global_uniforms(&self, queue: &wgpu::Queue, world: &World) {
        if let Some(camera_data) = world.get_resource::<CameraUniform>() {
            queue.write_buffer(&self.camera_buffer, 0, bytemuck::bytes_of(camera_data));
        }

        if let Some(light_data) = world.get_resource::<GlobalLightDataUniform>() {
            queue.write_buffer(&self.light_buffer, 0, bytemuck::bytes_of(light_data));
        }
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
            render_pass.set_bind_group(1, &self.light_bind_group, &[]);
            render_pass.set_bind_group(2, &material.bind_group, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..mesh.num_indices, 0, 0..1);
        }
    }

    fn create_uniform_resource<T>(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        label: &str,
    ) -> (wgpu::Buffer, wgpu::BindGroup) {
        let size = std::mem::size_of::<T>() as u64;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(&format!("{} Buffer", label)),
            size,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some(&format!("{} Bind Group", label)),
        });

        (buffer, bind_group)
    }


}
