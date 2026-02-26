use std::sync::Arc;
use winit::window::Window;
use engine_textures::Texture;
use engine_assets::AssetManager;
use engine_render::{PipelineBuilder, Renderer};


use crate::GameLogic;
// This will store the state of our game
pub struct State<T: GameLogic> {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    egui_ctx: egui::Context,
    pub egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
    pub game_logic: T,
    asset_manager: AssetManager,
    renderer: Renderer,
    depth_texture: Texture,
    pub window: Arc<Window>,
}

impl<T: GameLogic> State<T> {
    pub async fn new(window: Arc<Window>, mut game_logic: T) -> anyhow::Result<Self> {
        let size = window.inner_size();
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        //let limits = adapter.limits();
        //println!("Adapter limits: {:?}", limits);

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let modes = &surface_caps.present_modes;
        println!("Supported present modes: {:?}", modes);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox, // Vsync
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let renderer = Renderer::new(&device);

        let mut asset_manager = AssetManager::new(&device, &queue);
        let standard_pipeline = PipelineBuilder::build_standard_pipeline(&device, &config);
        asset_manager.pipeline_cache.insert("standard".to_string(), standard_pipeline);


        let egui_ctx = egui::Context::default();
        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            Some(device.limits().max_texture_dimension_2d as usize), 
        );
        let egui_renderer = egui_wgpu::Renderer::new(
            &device,
            config.format,
            egui_wgpu::RendererOptions::default(), 
        );

        game_logic.init(&device, &queue, &mut asset_manager);

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");
        
        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            egui_ctx,
            egui_state,
            egui_renderer,
            game_logic,
            asset_manager,
            renderer,
            depth_texture,
            window,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
            self.game_logic.on_resize(width, height);
            self.depth_texture = Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    pub fn update(&mut self) {
        self.game_logic.update();

        self.renderer.update_global_uniforms(&self.queue, &self.game_logic.world());

        self.sync_cursor_state();
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        if !self.is_surface_configured {
            return Ok(());
        }
        // puffin_egui::profiler_window(&self.egui_ctx);

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Render Encoder"),
        });

        let raw_input = self.egui_state.take_egui_input(&self.window);
        let full_output = self.egui_ctx.run(raw_input, |ctx| {
            self.game_logic.draw_ui(ctx);
        });

        let paint_jobs = self.egui_ctx.tessellate(full_output.shapes, full_output.pixels_per_point);
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: self.window.scale_factor() as f32,
        };

        for (id, delta) in &full_output.textures_delta.set {
            self.egui_renderer.update_texture(&self.device, &self.queue, *id, delta);
        }
        
        self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Game World Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            self.renderer.draw_world(
                &mut render_pass,
                &mut self.game_logic.world(),
                &self.asset_manager,
            );
        } 

        {
            let ui_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Egui UI Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let mut static_pass = ui_pass.forget_lifetime();
            self.egui_renderer.render(&mut static_pass, &paint_jobs, &screen_descriptor);
        } 

        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn sync_cursor_state(&self) {
        let visible = self.game_logic.is_cursor_visible();
        
        // 1. Sichtbarkeit
        self.window.set_cursor_visible(visible);

        // 2. Grab-Mode
        if visible {
            let _ = self.window.set_cursor_grab(winit::window::CursorGrabMode::None);
        } else {
            // Zuerst Locked versuchen (bestes Ergebnis für FPS)
            if self.window.set_cursor_grab(winit::window::CursorGrabMode::Locked).is_err() {
                // Falls das OS "Locked" ablehnt, versuchen wir "Confined"
                let _ = self.window.set_cursor_grab(winit::window::CursorGrabMode::Confined);
            }
        }
    }

}

