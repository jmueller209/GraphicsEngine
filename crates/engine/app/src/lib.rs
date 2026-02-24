pub mod state;
pub mod app;
pub mod ressources;
use engine_ecs::CameraUniform;
use engine_assets::AssetManager;

pub trait GameLogic {
    fn init(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, asset_manager: &mut AssetManager);
    fn update(&mut self);
    fn world(&mut self) -> &mut bevy_ecs::world::World;
    fn draw_ui(&mut self, ctx: &egui::Context);
    fn on_window_input(&mut self, event: &winit::event::WindowEvent, ui_consumed: bool);
    fn on_device_input(&mut self, event: &winit::event::DeviceEvent);
    fn on_resize(&mut self, width: u32, height: u32);
    fn get_primary_camera_uniform(&self) -> CameraUniform;
    fn is_cursor_visible(&self) -> bool;
}

