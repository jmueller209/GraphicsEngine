use engine_app::GameLogic;
use engine_camera::{Camera, CameraController};
use winit::event::{WindowEvent, KeyEvent, ElementState};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::ui::main_menu;
use crate::ui::hud;
use crate::ui::stats;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
}

pub struct Game {
    pub score: u32,
    pub speed: f32,
    pub state: GameState,
    pub main_camera: Camera,
    pub main_camera_controller: CameraController,
}

impl Game {
    // This is your dedicated constructor
    pub fn new() -> Self {
        Self {
            main_camera: Camera::new(),
            main_camera_controller: CameraController::new(),
            score: 0,
            speed: 0.0,
            state: GameState::MainMenu,
        }
    }
}

impl GameLogic for Game {

    fn camera_mut(&mut self) -> &mut Camera {
        &mut self.main_camera
    }

    fn camera(&self) -> &Camera {
        &self.main_camera
    }

    fn on_input(&mut self, event: &WindowEvent, ui_consumed: bool) {
        // We only care about Key Presses (not releases) for now
        if let WindowEvent::KeyboardInput {
            event: KeyEvent {
                state,
                physical_key: PhysicalKey::Code(keycode),
                ..
            },
            ..
        } = event 
        {
            // 2. Only pass the input to the controller if the UI didn't use it
            if !ui_consumed {
                // We convert the winit 'state' into a boolean: Pressed = true, Released = false
                let is_pressed = *state == ElementState::Pressed;
                
                self.main_camera_controller.handle_key(*keycode, is_pressed);
            }
        }
    }

    fn update(&mut self) {
        self.main_camera_controller.update_camera(&mut self.main_camera);
    }

    fn draw_ui(&mut self, ctx: &egui::Context) {
        stats::draw(ctx, self);

         egui::CentralPanel::default()
            .frame(egui::Frame::NONE) 
            .show(ctx, |ui| {
                
                match self.state {
                    GameState::MainMenu => main_menu::draw(ui, self),
                    GameState::Playing  => hud::draw(ui, self),
                    GameState::Paused   => {
                        // You could add a simple inline pause overlay here 
                        // or create a separate pause.rs file
                        ui.heading("PAUSED");
                        if ui.button("Resume").clicked() {
                            self.state = GameState::Playing;
                        }
                    }
                }
            });
    }
}
