use egui::{Color32, RichText, Align2, vec2};
use crate::game::Game;

pub fn draw(ctx: &egui::Context, _game: &Game) {
    // Area allows us to float the UI above the rest of the game
    egui::Area::new(egui::Id::new("fps_overlay"))
        // RIGHT_TOP pins it to the top-right corner.
        // The offset vec2(-10.0, 10.0) pushes it 10px left and 10px down.
        .anchor(Align2::RIGHT_TOP, vec2(-10.0, 10.0))
        .show(ctx, |ui| {
            egui::Frame::window(ui.style())
                .fill(Color32::from_rgba_premultiplied(0, 0, 0, 150))
                .stroke(egui::Stroke::NONE)
                .inner_margin(5.0)
                .corner_radius(5.0)
                .show(ui, |ui| {
                    
                    // Calculate raw FPS based on the last frame time according to egui
                    let fps = 1.0 / ui.input(|i| i.unstable_dt);
                    
                    let color = if fps > 120.0 {
                        Color32::from_rgb(0, 255, 255) // Cyan for high refresh
                    } else if fps > 55.0 {
                        Color32::GREEN
                    } else if fps > 30.0 {
                        Color32::YELLOW
                    } else {
                        Color32::RED
                    };

                    ui.label(
                        RichText::new(format!("FPS: {:.0}", fps))
                            .size(14.0)
                            .strong()
                            .color(color)
                    );
                });
        });
}
