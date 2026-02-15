use crate::game::Game;
use egui::{Color32, RichText, Vec2, Rect, Pos2, Align2, FontId, Stroke};

pub fn draw(ui: &mut egui::Ui, game: &mut Game) {
    // --- 1. Hintergrund ---
    // Wir füllen den gesamten verfügbaren Bereich mit einer dunklen Farbe
    let bg_rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(
        bg_rect,
        0.0,
        Color32::from_rgb(10, 10, 15) // Tiefes Dunkelblau/Schwarz
    );
    
    // --- 2. Der Titel (Oben Zentriert) ---
    ui.vertical_centered(|ui| {
        ui.add_space(50.0); // Abstand zum oberen Rand

        let title_text = "THE WHISPERING DARK";
        let title_font = FontId::proportional(60.0);
        
        // Schatten-Effekt (leicht versetzt)
        ui.painter().text(
            ui.max_rect().center_top() + Vec2::new(4.0, 54.0),
            Align2::CENTER_TOP,
            title_text,
            title_font.clone(),
            Color32::BLACK,
        );

        // Haupt-Text (Blutrot)
        ui.painter().text(
            ui.max_rect().center_top() + Vec2::new(0.0, 50.0),
            Align2::CENTER_TOP,
            title_text,
            title_font,
            Color32::from_rgb(180, 0, 0),
        );
    });

    // --- 3. Die Menü-Box (Manuelle Positionierung) ---
    let menu_width = 350.0;
    let menu_height = 400.0;
    let menu_rect = Rect::from_min_size(
        Pos2::new(80.0, 250.0), // 80px von links, 250px von oben
        Vec2::new(menu_width, menu_height)
    );

    // Hintergrund der Box (Halbtransparent)
    ui.painter().rect_filled(
        menu_rect,
        2.0,
        Color32::from_black_alpha(200),
    );
    
    // Rahmen der Box
    ui.painter().rect_stroke(
        menu_rect, 
        2.0, 
        Stroke::new(1.0, Color32::from_rgb(50, 50, 50)),
        egui::StrokeKind::Inside
    );

    // --- 4. Menü-Inhalt (Hier nutzen wir child_ui für egui 0.33) ---
    let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(menu_rect));

    child_ui.vertical_centered(|ui| {
        ui.add_space(40.0); // Polsterung oben innerhalb der Box

        // Horror-Button Helper (Closure)
        let horror_button = |ui: &mut egui::Ui, text: &str| -> bool {
            let btn_text = RichText::new(text)
                .size(32.0)
                .family(egui::FontFamily::Proportional)
                .color(Color32::from_rgb(200, 200, 200));

            let btn = egui::Button::new(btn_text)
                .frame(false) 
                .min_size(Vec2::new(200.0, 50.0));

            let response = ui.add(btn);

            // Hover-Effekt: Text leuchtet rot auf
            if response.hovered() {
                ui.painter().text(
                    response.rect.center(),
                    Align2::CENTER_CENTER,
                    text,
                    FontId::proportional(32.0),
                    Color32::from_rgb(255, 0, 0)
                );
            }
            
            response.clicked()
        };

        // --- Die eigentlichen Buttons ---
        if horror_button(ui, "START GAME") {
            game.set_state("playing");
        }
        ui.add_space(15.0);

        if horror_button(ui, "OPTIONS") {
            // Hier käme deine Options-Logik hin
        }
        ui.add_space(15.0);

        if horror_button(ui, "EXIT") {
            std::process::exit(0);
        }
    });
    
    // --- 5. Highscore / Footer (Unten Rechts) ---
    let footer_rect = ui.available_rect_before_wrap();
    let score_pos = footer_rect.right_bottom() - Vec2::new(180.0, 40.0);
    
    ui.painter().text(
        score_pos,
        Align2::LEFT_TOP,
        format!("High Score: {}", 1000),
        FontId::proportional(14.0),
        Color32::from_white_alpha(50), // Geisterhaft blass
    );
}
