use engine_assets::AssetManager;
use crate::Transform;
use bevy_ecs::prelude::*;
use crate::ecs_components::assets::*;

#[derive(Bundle)]
pub struct Sprite3DBundle {
    pub transform: Transform,
    pub mesh: MeshHandle,
    pub material: MaterialHandle,
}

impl Sprite3DBundle {
    pub fn new(
        mesh_name: &str, 
        material_name: &str, 
        position: glam::Vec3, 
        asset_manager: &AssetManager
    ) -> Self {
        Self {
            transform: Transform {
                position,
                rotation: glam::Quat::IDENTITY,
                scale: glam::Vec3::ONE,
            },
            mesh: MeshHandle(asset_manager.get_mesh_id(mesh_name)),
            material: MaterialHandle(asset_manager.get_material_id(material_name)),
        }
    }
}
