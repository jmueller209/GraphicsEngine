pub mod state;
pub mod app;
pub mod ressources;

pub trait GameLogic {
    fn update(&mut self);
    fn draw_ui(&mut self, ctx: &egui::Context);
    fn on_window_input(&mut self, event: &winit::event::WindowEvent, ui_consumed: bool);
    fn on_device_input(&mut self, event: &winit::event::DeviceEvent);
    fn on_resize(&mut self, width: u32, height: u32);
    fn get_primary_camera_uniform(&self) -> [[f32; 4]; 4];
    fn is_cursor_visible(&self) -> bool;
}

