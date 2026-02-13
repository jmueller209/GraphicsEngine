pub mod state;
pub mod app;

use winit::event::WindowEvent;


pub trait GameLogic {
    fn update(&mut self);
    fn draw_ui(&mut self, ctx: &egui::Context);
    fn on_input(&mut self, event: &WindowEvent, ui_consumed: bool);
}

