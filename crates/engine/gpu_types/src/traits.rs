pub trait BindGroupLayout {
    fn bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout;
}

pub trait BufferLayout {
    fn buffer_layout() -> wgpu::VertexBufferLayout<'static>;
}


