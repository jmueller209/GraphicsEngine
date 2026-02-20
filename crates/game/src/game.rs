use engine_app::GameLogic;
use engine_ecs::{ECSManager, FlyCameraBundle, ActionState, RawInputState, load_input_bindings, GameState, input_mapping_system, camera_matrix_system, fly_camera_controller_system, FrameContext, CameraUniform, sync_camera_uniform_system, StateDefinition, CameraSettings};
use winit::event::{WindowEvent, ElementState};
use winit::keyboard::{PhysicalKey};
use crate::ui::{main_menu, hud, pause_menu, stats};
use crate::systems::switch_game_state_system;
use bevy_ecs::prelude::*;



pub const STATE_CONFIGS: &[StateDefinition] = &[
    StateDefinition { name: "main_menu", cursor_visible: true },
    StateDefinition { name: "playing",   cursor_visible: false },
    StateDefinition { name: "paused",    cursor_visible: true },
];

pub const INITIAL_GAME_STATE: &'static str = "main_menu";

pub struct Game {
    pub ecs_manager : ECSManager,
    last_update: std::time::Instant,
    tick_count: u64,
    total_time: f64,
}

impl Game {
    // This is your dedicated constructor
    pub fn new() -> Self {
        // Creating the Ecs-Manager
        let mut ecs_manager = ECSManager::new();
       
        // Loading keybindings from a config file
        let keybindings = load_input_bindings("../ressources/keybindings/keybindings.json");
        println!("Loaded Keybindings: {:?}", keybindings);

        // Registering the keybinding ressources in the ECS
        ecs_manager.world.insert_resource(keybindings);
        ecs_manager.world.insert_resource(ActionState::default());
        ecs_manager.world.insert_resource(RawInputState::default());
        ecs_manager.world.insert_resource(GameState::new(STATE_CONFIGS, INITIAL_GAME_STATE));

        // Camera Uniform ressource
        ecs_manager.world.insert_resource(CameraUniform::default());
        
        // Spawning the camera entity with the FlyCameraBundle
        let camera = FlyCameraBundle::new();
        ecs_manager.world.spawn(camera);

        ecs_manager.schedule.add_systems((
            input_mapping_system, 
            camera_matrix_system, 
            fly_camera_controller_system,
            sync_camera_uniform_system,
            switch_game_state_system,
        ).chain());

        Self {
            ecs_manager: ecs_manager,
            last_update: std::time::Instant::now(),
            tick_count: 0,
            total_time: 0.0,
        }
    }

    pub fn state(&self) -> &str {
        let state_res = self.ecs_manager.world.resource::<engine_ecs::GameState>();
        &state_res.active_state
    }


    pub fn set_state(&mut self, new_state: &str) {
        let mut state_res = self.ecs_manager.world.resource_mut::<engine_ecs::GameState>();
        state_res.set_state(new_state);
    }

}

impl GameLogic for Game {
    fn on_device_input(&mut self, event: &winit::event::DeviceEvent) {
        let mut raw_input = self.ecs_manager.world.resource_mut::<RawInputState>();

        if let winit::event::DeviceEvent::Motion { axis, value } = event {
            // axis 0 ist X (horizontal), axis 1 ist Y (vertikal)
            if *axis == 0 {
                raw_input.mouse_delta.x += *value as f32;
            } else if *axis == 1 {
                raw_input.mouse_delta.y += *value as f32;
            }
        }
    }

    fn on_window_input(&mut self, event: &WindowEvent, ui_consumed: bool) {
        if ui_consumed {
            return;
        }

        let mut raw_input = self.ecs_manager.world.resource_mut::<RawInputState>();

        match event {
            // Keyboard-Events
            WindowEvent::KeyboardInput {
                event: key_event,
                ..
            } => {
                if let PhysicalKey::Code(keycode) = key_event.physical_key {
                    let is_pressed = key_event.state == ElementState::Pressed;
                    
                    if is_pressed {
                        raw_input.pressed_keys.insert(keycode);
                    } else {
                        raw_input.pressed_keys.remove(&keycode);
                    }
                }
            }

            // Mouse-Button-Events
            WindowEvent::MouseInput { state, button, .. } => {
                let is_pressed = *state == ElementState::Pressed;
                
                if is_pressed {
                    raw_input.mouse_buttons.insert(*button);
                } else {
                    raw_input.mouse_buttons.remove(button);
                }
            }

            _ => {}
        }
    }

    fn on_resize(&mut self, width: u32, height: u32) {
        let aspect_ratio = width as f32 / height as f32;
        let mut query = self.ecs_manager.world.query::<&mut CameraSettings>();
        for mut settings in query.iter_mut(&mut self.ecs_manager.world) {
            settings.aspect_ratio = aspect_ratio;
        }
    }

    fn update(&mut self) {
        // Calculate Delta Time
        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        self.tick_count += 1;
        self.total_time += dt as f64;

        let ctx = FrameContext {
            dt,
            tick: self.tick_count,
            total_time: self.total_time,
        };

        self.ecs_manager.update(ctx);

        // Zero out the mouse delta after processing to avoid accumulating it across frames
        let mut raw_input = self.ecs_manager.world.resource_mut::<RawInputState>();
        raw_input.mouse_delta = glam::Vec2::ZERO;

        // Update previous values in action state
        let mut action_state = self.ecs_manager.world.resource_mut::<ActionState>();
        action_state.update_previous();

        if !action_state.active_actions.is_empty() {
            println!("Tick: {} | Actions: {:?}", self.tick_count, action_state.active_actions);
        }
    }

    fn draw_ui(&mut self, ctx: &egui::Context) {
        stats::draw(ctx, self);

         egui::CentralPanel::default()
            .frame(egui::Frame::NONE) 
            .show(ctx, |ui| {
                
                match self.state() {
                    "main_menu" => main_menu::draw(ui, self),
                    "playing"  => hud::draw(ui),
                    "paused"   => pause_menu::draw(ui, self),
                    _ => {}
                }
            });
    }
    fn get_primary_camera_uniform(&self) -> [[f32; 4]; 4] {
            self.ecs_manager.world
                .get_resource::<CameraUniform>()
                .map(|b| b.camera_uniform)
                .unwrap_or(glam::Mat4::IDENTITY.to_cols_array_2d())
        }

    fn is_cursor_visible(&self) -> bool {
            self.ecs_manager.world
                .get_resource::<GameState>()
                .map(|s| s.is_cursor_visible())
                .unwrap_or(true)
        }
}
