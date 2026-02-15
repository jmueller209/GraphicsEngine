use crate::game::{Game, GameState};
use egui::{Color32, RichText, Vec2, Rect, Pos2, Align2, FontId, Sense, Stroke};

pub fn draw(ui: &mut egui::Ui, game: &mut Game) {
    // 1. Draw the Background Image
    // In a real engine, you would load the texture and pass the TextureId here.
    // For now, we simulate the "Dark Atmosphere" with a gradient if the image isn't loaded.
    let uv = Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0));
    
    // If you have the image texture loaded:
    // ui.painter().image(texture_id, ui.available_rect_before_wrap(), uv, Color32::WHITE);
    
    // Fallback: A dark, foggy vertical gradient
    let bg_rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(
        bg_rect,
        0.0,
        Color32::from_rgb(10, 10, 15) // Deep black/blue darkness
    );
    
    // 2. The Title (Centered Top)
    // We use a custom painter to create the "Shadow/Glow" effect of horror text
    ui.vertical_centered(|ui| {
        ui.add_space(50.0); // Space from top

        let title_text = "THE WHISPERING DARK";
        let title_font = FontId::proportional(60.0); // Use a serif/proportional font if possible
        
        // Draw Shadow (Black, offset)
        ui.painter().text(
            ui.max_rect().center_top() + Vec2::new(4.0, 54.0), // Slight offset
            Align2::CENTER_TOP,
            title_text,
            title_font.clone(),
            Color32::BLACK,
        );

        // Draw Main Text (Blood Red)
        ui.painter().text(
            ui.max_rect().center_top() + Vec2::new(0.0, 50.0),
            Align2::CENTER_TOP,
            title_text,
            title_font,
            Color32::from_rgb(180, 0, 0), // Blood Red
        );
    });

    // 3. The "Grunge" Menu Box (Left Side)
    // We position this manually to match the image layout (Left-Center)
    let menu_width = 350.0;
    let menu_height = 400.0;
    let menu_rect = Rect::from_min_size(
        Pos2::new(80.0, 250.0), // 80px from left, 250px from top
        Vec2::new(menu_width, menu_height)
    );

    // Draw the dark background for the menu options
    ui.painter().rect_filled(
        menu_rect,
        2.0, // Rough corners, not perfectly round
        Color32::from_black_alpha(200), // Semi-transparent black
    );
    
    // Add a rough border to the box
    ui.painter().rect_stroke(
        menu_rect, 
        2.0, 
        Stroke::new(1.0, Color32::from_rgb(50, 50, 50)),
        egui::StrokeKind::Inside
    );

    // Create a UI Rendering Scope inside that specific rectangle
    ui.allocate_ui_at_rect(menu_rect, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0); // Padding inside the box

            // --- Custom Horror Button Helper ---
            let horror_button = |ui: &mut egui::Ui, text: &str| -> bool {
                // We use "RichText" to make it look jagged/messy
                let btn_text = RichText::new(text)
                    .size(32.0)
                    .family(egui::FontFamily::Proportional) // Serif looks creepier
                    .color(Color32::from_rgb(200, 200, 200)); // Bone white

                // .frame(false) makes it look like text painted on the background
                let btn = egui::Button::new(btn_text)
                    .frame(false) 
                    .min_size(Vec2::new(200.0, 50.0));

                let response = ui.add(btn);

                // Hover Effect: Turn Red
                if response.hovered() {
                    ui.painter().text(
                        response.rect.center(),
                        Align2::CENTER_CENTER,
                        text,
                        FontId::proportional(32.0),
                        Color32::from_rgb(255, 0, 0) // Glow red on hover
                    );
                }
                
                response.clicked()
            };

            // --- MENU OPTIONS ---
            
            if horror_button(ui, "START GAME") {
                game.state = GameState::Playing;
            }
            ui.add_space(10.0);

            if horror_button(ui, "OPTIONS") {
                // Settings logic
            }
            ui.add_space(10.0);

            if horror_button(ui, "EXIT") {
                std::process::exit(0);
            }
        });
    });
    
    // 4. Subtle Stats (Hidden in corner, darker)
    let bottom_rect = ui.available_rect_before_wrap();
    ui.put(
        Rect::from_min_max(
            bottom_rect.right_bottom() - Vec2::new(150.0, 30.0), 
            bottom_rect.right_bottom()
        ), 
        |ui: &mut egui::Ui| {
            ui.label(RichText::new(format!("High Score: {}", game.score))
                .color(Color32::from_white_alpha(50)) // Ghostly faint
                .size(14.0)
            )
        }
    );
}
