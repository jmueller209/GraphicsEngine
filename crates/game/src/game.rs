use engine_app::GameLogic;
use engine_ecs::{ECSManager, FlyCameraBundle, Sprite3DBundle, fly_camera_controller_system, GameStateConfig, FrameContext, GameState};
use engine_assets::AssetManager;
use engine_gpu_types::CameraUniform;
use winit::event::WindowEvent;
use crate::ui::{main_menu, hud, pause_menu, stats};
use bevy_ecs::prelude::*;


pub const STATE_CONFIG: &[GameStateConfig] = &[
    GameStateConfig { name: "main_menu", cursor_visible: true },
    GameStateConfig { name: "playing",   cursor_visible: false },
    GameStateConfig { name: "paused",    cursor_visible: true },
];

pub const INTIAL_STATE: &str = "main_menu";

pub struct Game {
    pub ecs_manager : ECSManager,
    last_update: std::time::Instant,
    tick_count: u64,
    total_time: f64,
}

impl Game {
    pub fn new() -> Self {
        let mut ecs_manager = ECSManager::new();
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
    fn init(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, asset_manager: &mut AssetManager) {
        asset_manager.initialize_assets("../ressources/assets/asset_manifest.json", device, queue);
        self.ecs_manager.load_input_bindings("../ressources/keybindings/keybindings.json");
        self.ecs_manager.set_game_state_config(STATE_CONFIG, INTIAL_STATE);
        self.ecs_manager.set_ambient_light_color([1.0, 1.0, 1.0, 1.0]);

        let camera = FlyCameraBundle::new();
        let cube = Sprite3DBundle::new(
            "cube_mesh",
            "cube_material",
            glam::Vec3::new(0.0, 0.0, 0.0), 
            asset_manager
        );

        self.ecs_manager.world.spawn(cube);
        self.ecs_manager.world.spawn(camera);

        self.ecs_manager.schedule.add_systems((
            fly_camera_controller_system,
        ));

    }

    fn on_device_input(&mut self, event: &winit::event::DeviceEvent) {
        self.ecs_manager.on_device_input(event);
    }

    fn on_window_input(&mut self, event: &WindowEvent, ui_consumed: bool) {
        self.ecs_manager.on_window_input(event, ui_consumed);
    }

    fn on_resize(&mut self, width: u32, height: u32) {
       self.ecs_manager.on_resize(width, height); 
    }

    fn world(&mut self) -> &mut World {
        &mut self.ecs_manager.world
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

    fn get_primary_camera_uniform(&self) -> CameraUniform {
        self.ecs_manager.world
            .get_resource::<CameraUniform>()
            .cloned()
            .unwrap_or_else(|| CameraUniform::new())
    }

    fn is_cursor_visible(&self) -> bool {
            self.ecs_manager.world
                .get_resource::<GameState>()
                .map(|s| s.is_cursor_visible())
                .unwrap_or(true)
        }
}
