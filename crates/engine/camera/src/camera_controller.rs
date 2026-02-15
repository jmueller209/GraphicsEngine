use winit::keyboard::KeyCode;
use crate::camera::Camera; // Adjust this path based on your crate structure

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new() -> Self {
        Self {
            speed: 0.05,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    /// Returns true if the key was handled by the controller
    pub fn handle_key(&mut self, code: KeyCode, is_pressed: bool) -> bool {
        println!("Key event: {:?}, pressed: {}", code, is_pressed);
        match code {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.is_forward_pressed = is_pressed;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.is_left_pressed = is_pressed;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.is_backward_pressed = is_pressed;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.is_right_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

  pub fn update_camera(&self, camera: &mut Camera) {
        // 1. Get current eye position (assuming a getter exists)
        let mut current_eye = camera.eye(); 
        
        // 2. Calculate the 'Forward' vector
        let forward = camera.target() - current_eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.length();

        // 3. Move Forward/Backward
        if self.is_forward_pressed && forward_mag > self.speed {
            current_eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            current_eye -= forward_norm * self.speed;
        }

        // 4. Calculate 'Right' vector for orbiting
        let right = forward_norm.cross(camera.up());

        // 5. Orbit Left/Right
        // Re-calculate forward based on potential move above to keep math accurate
        let forward = camera.target() - current_eye;
        let forward_mag = forward.length();

        if self.is_right_pressed {
            current_eye = camera.target() - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            current_eye = camera.target() - (forward - right * self.speed).normalize() * forward_mag;
        }
        
        // 6. Push the final calculated position back to the camera
        camera.set_eye(current_eye);
        
        // This ensures the view_proj matrix is recalculated
        camera.build_view_projection_matrix();
    }}
