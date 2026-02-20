use crate::game::Game;
use egui::{Color32, RichText, Align2, FontId};

pub fn draw(ui: &mut egui::Ui, game: &mut Game) {
    // --- 1. Abdunkelndes Overlay ---
    // Wir legen einen dunklen Schleier über das laufende Spiel
    let screen_rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(
        screen_rect,
        0.0,
        Color32::from_black_alpha(180) // 180 für gute Lesbarkeit des Menüs
    );
    
    // --- 2. Titel "PAUSED" ---
    ui.vertical_centered(|ui| {
        ui.add_space(100.0);
        let title_font = egui::FontId::proportional(50.0);
        ui.painter().text(
            ui.max_rect().center_top() + egui::Vec2::new(0.0, 100.0),
            egui::Align2::CENTER_TOP,
            "GAME PAUSED",
            title_font,
            Color32::from_rgb(200, 200, 200),
        );
    });

    // --- 3. Die Menü-Box (Zentriert) ---
    let menu_width = 300.0;
    let menu_height = 300.0;
    let center = screen_rect.center();
    
    let menu_rect = egui::Rect::from_center_size(
        center, 
        egui::Vec2::new(menu_width, menu_height)
    );

    // Box-Hintergrund
    ui.painter().rect_filled(menu_rect, 5.0, Color32::from_black_alpha(220));
    ui.painter().rect_stroke(menu_rect, 5.0, egui::Stroke::new(1.0, Color32::GRAY), egui::StrokeKind::Inside);

    // --- 4. Inhalt ---
    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(menu_rect));

    child_ui.vertical_centered(|ui| {
        ui.add_space(30.0);

        // Wir nutzen den gleichen Button-Stil wie im Hauptmenü
        let pause_button = |ui: &mut egui::Ui, text: &str| -> bool {
            let btn = egui::Button::new(RichText::new(text).size(28.0).color(Color32::LIGHT_GRAY))
                .frame(false);
            
            let response = ui.add(btn);
            
            // Hover-Effekt (Rot wie im Main Menu)
            if response.hovered() {
                ui.painter().text(
                    response.rect.center(),
                    Align2::CENTER_CENTER,
                    text,
                    FontId::proportional(28.0),
                    Color32::from_rgb(180, 0, 0)
                );
            }
            response.clicked()
        };

        if pause_button(ui, "RESUME") {
            game.set_state("playing");
        }

        ui.add_space(20.0);

        if pause_button(ui, "MAIN MENU") {
            game.set_state("main_menu");
        }

        ui.add_space(20.0);

        if pause_button(ui, "QUIT") {
            std::process::exit(0);
        }
    });
}
