use engine_app::GameLogic;
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
}

impl Game {
    // This is your dedicated constructor
    pub fn new() -> Self {
        Self {
            score: 0,
            speed: 0.0,
            state: GameState::MainMenu,
        }
    }
}

impl GameLogic for Game {
    
    fn on_input(&mut self, event: &WindowEvent, ui_consumed: bool) {
        // We only care about Key Presses (not releases) for now
        if let WindowEvent::KeyboardInput {
            event: KeyEvent {
                state: ElementState::Pressed,
                physical_key: PhysicalKey::Code(keycode),
                ..
            },
            ..
        } = event 
        {
            // CATEGORY 1: Global Keys
            // These run even if the UI 'consumed' the event.
            // (e.g., pressing ESC to close a menu or pause)
            match keycode {
                KeyCode::Escape => {
                    // Toggle between Playing and Paused
                    self.state = match self.state {
                        GameState::Playing => GameState::Paused,
                        GameState::Paused => GameState::Playing,
                        GameState::MainMenu => GameState::MainMenu, // Don't unpause a main menu
                    };
                    println!("State switched to: {:?}", self.state);
                }
                KeyCode::F3 => {
                    println!("Debug info toggled (Example Global Key)");
                }
                _ => {}
            }

            // CATEGORY 2: Gameplay Keys
            // These ONLY run if the UI did NOT consume the event.
            // This prevents your character from moving while you type in a text box.
            if !ui_consumed && self.state == GameState::Playing {
                match keycode {
                    KeyCode::Space => {
                        println!("Jump!");
                        self.speed += 5.0; 
                    }
                    KeyCode::KeyW => {
                        println!("Moving Forward");
                        self.speed += 0.5;
                    }
                    KeyCode::KeyS => {
                         println!("Braking");
                        self.speed = (self.speed - 0.5).max(0.0);
                    }
                    _ => {}
                }
            }
        }
    }

    fn update(&mut self) {
        // TODO
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
