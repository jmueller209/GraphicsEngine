use crate::game::Game;

pub fn draw(ui: &mut egui::Ui, game: &Game) {
    // A small overlay in the corner during gameplay
    egui::Frame::popup(ui.style()).show(ui, |ui| {
        ui.label(format!("Speed: {:.1}", 1.0)); // Placeholder for actual speed
    });
}
