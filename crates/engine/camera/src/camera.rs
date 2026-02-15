use glam::{Mat4, Vec3};
use engine_gpu_types::CameraUniform;

pub struct Camera {
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    aspect_ratio: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
    view_proj: Mat4,
    has_changed: bool,
}

impl Camera {
    pub fn new() -> Self {
        let mut camera = Self {
            eye: Vec3::new(0.0, 2.0, 2.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            aspect_ratio: 16.0 / 9.0,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
            view_proj: Mat4::IDENTITY,
            has_changed: true,
        };
        // Pre-calculate the first matrix
        camera.build_view_projection_matrix();
        camera
    }

    // Typical Rust setter
    pub fn set_eye(&mut self, new_eye: Vec3) {
        self.eye = new_eye;
        self.has_changed = true;
    }

    pub fn eye(&self) -> Vec3 {
        self.eye
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.has_changed = true;
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn set_target(&mut self, new_target: Vec3) {
        self.target = new_target;
        self.has_changed = true;
    }

    pub fn target(&self) -> Vec3 {
        self.target
    }

    pub fn set_up(&mut self, new_up: Vec3) {
        self.up = new_up;
        self.has_changed = true;
    }

    pub fn up(&self) -> Vec3 {
        self.up
    }

    // Notice the &mut self here! We need it to update the cache.
    pub fn build_view_projection_matrix(&mut self) -> Mat4 {
        if self.has_changed {
            let view = Mat4::look_at_rh(self.eye, self.target, self.up);
            let proj = Mat4::perspective_rh(
                self.fovy.to_radians(),
                self.aspect_ratio,
                self.znear,
                self.zfar,
            );

            self.view_proj = proj * view;
            self.has_changed = false;
        }
        self.view_proj
    }
    pub fn update_uniform(&mut self, uniform: &mut CameraUniform) {
        let matrix = self.build_view_projection_matrix();
        uniform.view_proj = matrix.to_cols_array_2d();
    }
}
