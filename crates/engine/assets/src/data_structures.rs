


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
