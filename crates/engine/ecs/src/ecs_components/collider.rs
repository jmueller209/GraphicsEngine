use bevy_ecs::prelude::*;

pub enum ColliderShape {
    Sphere { radius: f32 },
    Cuboid { half_extents: glam::Vec3 }, // "Half-extents" is half the width/height/depth
}

#[derive(Component)]
pub struct Collider {
    pub shape: ColliderShape,
    pub is_solid: bool, // true = Wall, false = Checkpoint
}

// Helper to create a collider
impl Collider {
    pub fn box_collider(width : f32, height : f32, depth : f32, is_solid : bool) -> Self {
        Self {
            shape: ColliderShape::Cuboid { 
                half_extents: glam::Vec3::new(width / 2.0, height / 2.0, depth / 2.0) 
            },
            is_solid,
        }
    }

    pub fn spherical_collider(radius: f32, is_solid : bool) -> Self {
        Self {
            shape: ColliderShape::Sphere { radius },
            is_solid,
        }
    }
}
