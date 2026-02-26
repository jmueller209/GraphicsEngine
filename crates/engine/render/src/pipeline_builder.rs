use engine_gpu_types::{MaterialUniform, VertexPTN, CameraUniform, GlobalLightDataUniform, BufferLayout, BindGroupLayout};

pub struct PipelineBuilder;

impl PipelineBuilder {
    pub fn build_standard_pipeline(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/standard.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Standard Render Pipeline Layout"),
            bind_group_layouts: &[
                &CameraUniform::bind_group_layout(device),
                &GlobalLightDataUniform::bind_group_layout(device),
                &MaterialUniform::bind_group_layout(device),
            ],
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Standard Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            
            vertex: wgpu::VertexState {
                compilation_options: Default::default(),
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[VertexPTN::buffer_layout()],
            },

            fragment: Some(wgpu::FragmentState {
                compilation_options: Default::default(),
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },

            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),

            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }

}
