pub mod state;
pub mod app;

use winit::event::WindowEvent;
use engine_camera::Camera;


pub trait GameLogic {
    fn camera_mut(&mut self) -> &mut Camera; 
    fn camera(&self) -> &Camera;
    fn update(&mut self);
    fn draw_ui(&mut self, ctx: &egui::Context);
    fn on_input(&mut self, event: &WindowEvent, ui_consumed: bool);
}

