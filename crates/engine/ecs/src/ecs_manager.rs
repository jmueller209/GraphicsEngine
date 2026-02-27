use bevy_ecs::prelude::*;
use crate::ecs_components::{CameraSettings};
use crate::ecs_resources::{ActionState, RawInputState, InputBindings, GameState, GameStateConfig, FrameContext};
use crate::ecs_systems::{input_mapping_system, camera_matrix_system, sync_camera_uniform_system, sync_lights_uniform_system, input_clean_up_system};
use engine_gpu_types::{CameraUniform, GlobalLightDataUniform};
use winit::event::{WindowEvent, ElementState};
use winit::keyboard::{PhysicalKey};

pub struct ECSManager {
    pub world: World,
    pub schedule: Schedule,
}

#[derive(Resource, Default)]
pub struct GameTime {
    pub delta_seconds: f32,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EngineSet {
    Input,
    Logic,
    Sync,
    Cleanup,
}

impl ECSManager {
    pub fn new() -> Self {
        let mut world = World::new();
        let mut schedule = Schedule::default();
        
        world.insert_resource(GameTime::default());
        world.insert_resource(ActionState::default());
        world.insert_resource(RawInputState::default());
        world.insert_resource(InputBindings::default());
        world.insert_resource(CameraUniform::default());
        world.insert_resource(GlobalLightDataUniform::default());
        world.insert_resource(GameState::default());

        schedule.configure_sets((
            EngineSet::Input,
            EngineSet::Logic,
            EngineSet::Sync,
            EngineSet::Cleanup,
        ).chain());

        schedule.add_systems((
            input_mapping_system.in_set(EngineSet::Input),
            camera_matrix_system.in_set(EngineSet::Sync),
            sync_camera_uniform_system.in_set(EngineSet::Sync),
            sync_lights_uniform_system.in_set(EngineSet::Sync),
            input_clean_up_system.in_set(EngineSet::Cleanup),
        ));

        Self { world, schedule }
    }

    pub fn load_input_bindings(&mut self, path: &str){
        let mut input_bindings = self.world.resource_mut::<InputBindings>();
        input_bindings.load_from_file(path);
    }

    pub fn set_game_state_config(&mut self, config: &[GameStateConfig], initial_state: &str) {
        let mut game_state = self.world.resource_mut::<GameState>();
        game_state.set_config(config, initial_state);
    }

    pub fn set_ambient_light_color(&mut self, color: [f32; 4]) {
        let mut light_uniform = self.world.resource_mut::<GlobalLightDataUniform>();
        light_uniform.ambient_color = color;
    }

    pub fn update(&mut self, ctx: FrameContext) {
        self.world.insert_resource(ctx);
        self.schedule.run(&mut self.world);
    }

    pub fn on_device_input(&mut self, event: &winit::event::DeviceEvent) {
        let mut raw_input = self.world.resource_mut::<RawInputState>();

        if let winit::event::DeviceEvent::Motion { axis, value } = event {
            if *axis == 0 {
                raw_input.mouse_delta.x += *value as f32;
            } else if *axis == 1 {
                raw_input.mouse_delta.y += *value as f32;
            }
        }
    }

    pub fn on_window_input(&mut self, event: &WindowEvent, ui_consumed: bool) {
        if ui_consumed {
            return;
        }

        let mut raw_input = self.world.resource_mut::<RawInputState>();

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

    pub fn on_resize(&mut self, width: u32, height: u32) {
        let aspect_ratio = width as f32 / height as f32;
        let mut query = self.world.query::<&mut CameraSettings>();
        for mut settings in query.iter_mut(&mut self.world) {
            settings.aspect_ratio = aspect_ratio;
        }
    }
}
