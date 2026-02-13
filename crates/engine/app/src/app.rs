use std::sync::Arc;
use winit::event::{WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::application::ApplicationHandler;
use winit::window::Window;
use crate::state::State;
use crate::GameLogic;

pub struct App<T: GameLogic>{
    state: Option<State<T>>,
    game_logic: Option<T>,
}

impl <T: GameLogic> App<T> {
    pub fn new(game_logic: T) -> Self {
        Self {
            state: None,
            game_logic: Some(game_logic),
        }
    }
}

impl<T: GameLogic> ApplicationHandler for App<T>{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        //println!("Event loop control flow: {:?}", event_loop.control_flow()); 
        if let Some(logic) = self.game_logic.take() {
             // Ensure State::new now accepts 'logic' as an argument
            let state = pollster::block_on(State::new(window, logic)).unwrap();
            self.state = Some(state);
        }
    }
   
    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };
        
        // Get the response from egui for this event
        let response = state.egui_state.on_window_event(&state.window, &event);
        
        // Pass the event and whether it was consumed by the UI to the game logic
        state.game_logic.on_input(&event, response.consumed);
        
        // If the UI consumed the event, we don't want to process it further
        if response.consumed {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("Window close requested");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => state.resize(size.width, size.height),
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if it's lost or outdated
                    Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                        let size = state.window.inner_size();
                        state.resize(size.width, size.height);
                    }
                    Err(e) => {
                        log::error!("Unable to render {}", e);
                    }
                }
            }
            _ => {}
        }
    }
}
