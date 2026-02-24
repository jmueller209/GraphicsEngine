use serde::Deserialize;
use std::collections::HashMap;
use std::mem;

// Structs for deserializing the asset manifest JSON
#[derive(Deserialize, Debug, Clone)]
pub struct AssetManifest {
    pub textures: HashMap<String, String>,
    pub meshes: HashMap<String, String>,
    pub materials: HashMap<String, MaterialConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MaterialConfig {
    pub pipeline: String,
    pub diffuse: String,
    pub normal: Option<String>,
    pub roughness: f32,
    pub metallic: f32,
}


// Structs for managing loaded assets

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureId(pub usize);


pub struct MeshBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}


#[derive(Debug, Clone)]
pub struct MaterialData {
    pub pipeline_name: String,
    pub bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct MaterialUniforms {
    pub roughness: f32,
    pub metallic: f32,
    pub _padding: [f32; 2],
}


pub trait BufferLayout {
    fn buffer_layout() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexPTN {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}


impl BufferLayout for VertexPTN {
    fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<VertexPTN>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
