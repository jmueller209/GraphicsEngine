use glam::{Vec3, Quat, Mat4};
use engine_gpu_types::InstanceRaw;

pub struct Instance {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        let matrix = Mat4::from_rotation_translation(self.rotation, self.position);
        
        InstanceRaw {
            model: matrix.to_cols_array_2d(),
        }
    }
}
