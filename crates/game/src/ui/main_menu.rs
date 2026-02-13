use crate::game::{Game, GameState};
use egui::{Color32, RichText, Vec2};

pub fn draw(ui: &mut egui::Ui, game: &mut Game) {
    // 1. Create a "Full Screen" background frame
    let background_frame = egui::Frame::default()
        .fill(Color32::from_rgb(15, 18, 25)) // Dark professional background
        .inner_margin(20.0);

    background_frame.show(ui, |ui| {
        // Force the UI to take up the full available width/height of the panel
        // This ensures the background color covers everything
        ui.set_min_size(ui.available_size());

        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() * 0.15); // Dynamic top spacing

            // 2. Main Title with Glow/Emphasis
            ui.heading(
                RichText::new("PROJECT NEON")
                    .size(80.0)
                    .strong()
                    .color(Color32::from_rgb(0, 255, 200)) // Neon Teal
                    // FIX 1: Use extra_letter_spacing
                    // Note: If this still errors, remove the line (it's purely cosmetic)
                    // .extra_letter_spacing(4.0) 
            );

            ui.add_space(10.0);
            ui.label(RichText::new("v0.1.0 Alpha Build").weak().size(16.0));

            ui.add_space(50.0);

            // 3. Stats Section in a subtle sub-frame
            egui::Frame::canvas(ui.style())
                .fill(Color32::from_rgba_premultiplied(255, 255, 255, 5))
                // FIX 2: Use corner_radius instead of rounding
                .corner_radius(8.0)
                .show(ui, |ui| {
                    // Create a fixed-width box for the stats
                    ui.allocate_ui(Vec2::new(300.0, 80.0), |ui| {
                         ui.vertical_centered(|ui| {
                            ui.add_space(10.0);
                            ui.label(RichText::new("CURRENT TOP SCORE").strong().size(12.0).color(Color32::GRAY));
                            ui.add_space(5.0);
                            ui.label(RichText::new(format!("{}", game.score)).size(32.0).strong().color(Color32::WHITE));
                            ui.add_space(10.0);
                        });
                    });
                });

            ui.add_space(80.0);

            // 4. Large Action Buttons
            let btn_size = Vec2::new(280.0, 60.0);
            let start_text = RichText::new("START ENGINE").size(22.0).strong();

            // FIX 3: Use corner_radius on the button
            if ui.add_sized(btn_size, egui::Button::new(start_text).corner_radius(5.0)).clicked() {
                game.state = GameState::Playing;
            }

            ui.add_space(20.0);

            if ui.add_sized(Vec2::new(280.0, 40.0), egui::Button::new("SETTINGS").corner_radius(5.0)).clicked() {
                // Future settings logic
            }

            ui.add_space(20.0);

            if ui.add_sized(Vec2::new(280.0, 40.0), egui::Button::new("EXIT GAME").corner_radius(5.0)).clicked() {
                std::process::exit(0);
            }
        });
    });
}
