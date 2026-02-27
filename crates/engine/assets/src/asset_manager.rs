use std::collections::HashMap;
use std::f32::consts::PI;
use std::path::{Path, PathBuf};
use image::GenericImageView;
use wgpu::util::DeviceExt;
use crate::data_structures::{MaterialData, MeshBuffers,  MeshId, MaterialId, TextureId};
use engine_gpu_types::{VertexPTN, MaterialUniform};
use serde::Deserialize;

// Structs for deserializing the asset manifest JSON
#[derive(Deserialize, Debug, Clone)]
pub struct AssetManifest {
    pub textures: HashMap<String, String>,
    pub meshes: HashMap<String, String>,
    pub materials: HashMap<String, MaterialConfig>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MaterialConfig {
    pub pipeline: String,
    pub diffuse: String,
    pub normal: Option<String>,
    pub roughness: f32,
    pub metallic: f32,
}

pub struct AssetManager {
    meshes: Vec<MeshBuffers>,
    materials: Vec<MaterialData>,
    texture_views: Vec<wgpu::TextureView>, 

    mesh_registry: HashMap<String, MeshId>,
    material_registry: HashMap<String, MaterialId>,
    texture_registry: HashMap<String, TextureId>,

    pub default_sampler: wgpu::Sampler,
    pub default_normal_view: wgpu::TextureView,
    pub pipeline_cache: HashMap<String, wgpu::RenderPipeline>,
}

impl AssetManager {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let default_sampler = Self::create_default_sampler(device);
        let default_normal_view = Self::create_default_normal_view(device, queue);

        Self {
            meshes: Vec::new(),
            materials: Vec::new(),
            texture_views: Vec::new(),

            mesh_registry: HashMap::new(),
            material_registry: HashMap::new(),
            texture_registry: HashMap::new(),

            default_sampler,
            default_normal_view,
            pipeline_cache: HashMap::new(),
        }
    }

   pub fn get_mesh_id(&self, name: &str) -> MeshId {
        *self.mesh_registry.get(name).expect(&format!("Mesh '{}' could not be loaded.", name))
    }

    pub fn get_material_id(&self, name: &str) -> MaterialId {
        *self.material_registry.get(name).expect(&format!("Material '{}' could not be loaded.", name))
    }

    pub fn get_mesh(&self, id: MeshId) -> &MeshBuffers {
        &self.meshes[id.0]
    }

    pub fn get_material(&self, id: MaterialId) -> &MaterialData {
        &self.materials[id.0]
    }

    fn load_internal_assets(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.create_cube_mesh("internal:cube", device);
        self.create_sphere_mesh("internal:sphere", device, 0.5, 16, 32);
        self.create_single_color_material(
            "internal:white",
            [255, 255, 255, 255],
            0.5,
            0.0,
            device,
            queue,
        );
    }

    pub fn initialize_assets(&mut self, manifest_path: &str, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.clear_assets();
        self.load_internal_assets(device, queue);

        let file_content = std::fs::read_to_string(manifest_path).expect("Manifest not found");
        let manifest: AssetManifest = serde_json::from_str(&file_content).expect("JSON error");
        let base_path = Path::new(manifest_path).parent().unwrap_or(Path::new(""));

        for (name, rel_path) in &manifest.textures {
            let full_path = base_path.join(rel_path);
            let view = self.load_texture_from_path(full_path, device, queue);
            
            let id = TextureId(self.texture_views.len());
            self.texture_views.push(view);
            self.texture_registry.insert(name.clone(), id);
        }

        for (name, rel_path) in &manifest.meshes {
            let full_path = base_path.join(rel_path);
            let mesh_data = self.load_mesh_from_path(full_path, device);
            
            let id = MeshId(self.meshes.len());
            self.meshes.push(mesh_data);
            self.mesh_registry.insert(name.clone(), id);
        }

        let mat_configs: Vec<(String, MaterialConfig)> = manifest.materials.into_iter().collect();
        for (name, config) in mat_configs {
            self.create_material(&name, &config, device);
        }
    }

    pub fn create_material(&mut self, name: &str, config: &MaterialConfig, device: &wgpu::Device) {
        let pipeline = self.pipeline_cache.get(&config.pipeline)
            .expect(&format!("Pipeline '{}' missing.", config.pipeline));
        
        let layout = pipeline.get_bind_group_layout(2); // Material bind group is at index 2

        let diffuse_id = self.texture_registry.get(&config.diffuse)
            .expect(&format!("Diffuse Texture '{}' for material '{}' missing.", config.diffuse, name));
        let diffuse_view = &self.texture_views[diffuse_id.0];

        let normal_view = if let Some(normal_name) = &config.normal {
            let n_id = self.texture_registry.get(normal_name).expect(&format!("Normal Texture '{}' for material '{}' missing.", normal_name, name));
            &self.texture_views[n_id.0]
        } else {
            &self.default_normal_view
        };

        let uniforms = MaterialUniform {
            roughness: config.roughness,
            metallic: config.metallic,
            _padding: [0.0; 2],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Uniforms", name)),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(diffuse_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.default_sampler) },
                wgpu::BindGroupEntry { binding: 2, resource: uniform_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(normal_view)},
            ],
            label: Some(&format!("BG: {}", name)),
        });

        let id = MaterialId(self.materials.len());
        self.materials.push(MaterialData {
            pipeline_name: config.pipeline.clone(),
            bind_group,
        });
        self.material_registry.insert(name.to_string(), id);
    }


    fn load_texture_from_path(&self, path: PathBuf, device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::TextureView {
        let img = image::open(&path).expect("Image could not be loaded");
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        
        let format = if file_name.contains("_n.") || file_name.contains("_data.") {
            wgpu::TextureFormat::Rgba8Unorm // Color data is not in sRGB space for normal maps or
            // data textures
        } else {
            wgpu::TextureFormat::Rgba8UnormSrgb // Regular color textures in sRGB space
        };

        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(file_name),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn load_mesh_from_path(&self, path: PathBuf, device: &wgpu::Device) -> MeshBuffers {
        let (models, _) = tobj::load_obj(&path, &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        }).expect("OBJ Error");

        let m = &models[0];
        let vertices = (0..m.mesh.positions.len() / 3)
            .map(|i| VertexPTN {
                position: [m.mesh.positions[i*3], m.mesh.positions[i*3+1], m.mesh.positions[i*3+2]],
                tex_coords: [m.mesh.texcoords[i*2], 1.0 - m.mesh.texcoords[i*2+1]],
                normal: if m.mesh.normals.is_empty() { [0.0; 3] } else { 
                    [m.mesh.normals[i*3], m.mesh.normals[i*3+1], m.mesh.normals[i*3+2]] 
                },
            })
            .collect::<Vec<_>>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&m.mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        MeshBuffers { vertex_buffer, index_buffer, num_indices: m.mesh.indices.len() as u32 }
    }

    fn clear_assets(&mut self) {
        self.meshes.clear();
        self.materials.clear();
        self.texture_views.clear();
        self.mesh_registry.clear();
        self.material_registry.clear();
        self.texture_registry.clear();
    }

    fn create_default_sampler(device: &wgpu::Device) -> wgpu::Sampler {
        device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        })
    }

    fn create_single_pixel_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        rgba: [u8; 4],
        format: wgpu::TextureFormat,
        label: &str,
    ) -> wgpu::TextureView {
        let size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            size,
        );

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn create_default_normal_view(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::TextureView {
        Self::create_single_pixel_texture(
            device,
            queue,
            [128, 128, 255, 255], // Default normal pointing straight up
            wgpu::TextureFormat::Rgba8Unorm,
            "Default Normal Texture",
        )
    }

    pub fn create_single_color_material(
        &mut self,
        material_name: &str,
        rgba: [u8; 4],
        roughness: f32,
        metallic: f32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let tex_name = format!("{}_diffuse", material_name);

        let view = Self::create_single_pixel_texture(
            device, 
            queue, 
            rgba, 
            wgpu::TextureFormat::Rgba8UnormSrgb, 
            &tex_name
        );
        self.texture_registry.insert(tex_name.clone(), TextureId(self.texture_views.len()));
        self.texture_views.push(view);

        let config = MaterialConfig {
            pipeline: "standard".to_string(),
            diffuse: tex_name, 
            normal: None,
            roughness,
            metallic,
        };

        self.create_material(material_name, &config, device);
    }

    pub fn create_cube_mesh(&mut self, mesh_name: &str, device: &wgpu::Device){
        let vertices = vec![
            // ================= FRONT FACE (+Z) =================
            // Normal: [0.0, 0.0, 1.0]
            VertexPTN { position: [-0.5, -0.5,  0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0] }, // Unten Links
            VertexPTN { position: [ 0.5, -0.5,  0.5], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0] }, // Unten Rechts
            VertexPTN { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0] }, // Oben Rechts
            VertexPTN { position: [-0.5,  0.5,  0.5], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0] }, // Oben Links

            // ================= BACK FACE (-Z) =================
            // Normal: [0.0, 0.0, -1.0]
            VertexPTN { position: [ 0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0] }, // Unten Links (von hinten betrachtet)
            VertexPTN { position: [-0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0] }, // Unten Rechts
            VertexPTN { position: [-0.5,  0.5, -0.5], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0] }, // Oben Rechts
            VertexPTN { position: [ 0.5,  0.5, -0.5], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0] }, // Oben Links

            // ================= TOP FACE (+Y) =================
            // Normal: [0.0, 1.0, 0.0]
            VertexPTN { position: [-0.5,  0.5,  0.5], tex_coords: [0.0, 1.0], normal: [0.0, 1.0, 0.0] }, // Unten Links (auf der Fläche stehend)
            VertexPTN { position: [ 0.5,  0.5,  0.5], tex_coords: [1.0, 1.0], normal: [0.0, 1.0, 0.0] }, // Unten Rechts
            VertexPTN { position: [ 0.5,  0.5, -0.5], tex_coords: [1.0, 0.0], normal: [0.0, 1.0, 0.0] }, // Oben Rechts
            VertexPTN { position: [-0.5,  0.5, -0.5], tex_coords: [0.0, 0.0], normal: [0.0, 1.0, 0.0] }, // Oben Links

            // ================= BOTTOM FACE (-Y) =================
            // Normal: [0.0, -1.0, 0.0]
            VertexPTN { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], normal: [0.0, -1.0, 0.0] }, 
            VertexPTN { position: [ 0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], normal: [0.0, -1.0, 0.0] }, 
            VertexPTN { position: [ 0.5, -0.5,  0.5], tex_coords: [1.0, 0.0], normal: [0.0, -1.0, 0.0] }, 
            VertexPTN { position: [-0.5, -0.5,  0.5], tex_coords: [0.0, 0.0], normal: [0.0, -1.0, 0.0] }, 

            // ================= RIGHT FACE (+X) =================
            // Normal: [1.0, 0.0, 0.0]
            VertexPTN { position: [ 0.5, -0.5,  0.5], tex_coords: [0.0, 1.0], normal: [1.0, 0.0, 0.0] }, 
            VertexPTN { position: [ 0.5, -0.5, -0.5], tex_coords: [1.0, 1.0], normal: [1.0, 0.0, 0.0] }, 
            VertexPTN { position: [ 0.5,  0.5, -0.5], tex_coords: [1.0, 0.0], normal: [1.0, 0.0, 0.0] }, 
            VertexPTN { position: [ 0.5,  0.5,  0.5], tex_coords: [0.0, 0.0], normal: [1.0, 0.0, 0.0] }, 

            // ================= LEFT FACE (-X) =================
            // Normal: [-1.0, 0.0, 0.0]
            VertexPTN { position: [-0.5, -0.5, -0.5], tex_coords: [0.0, 1.0], normal: [-1.0, 0.0, 0.0] }, 
            VertexPTN { position: [-0.5, -0.5,  0.5], tex_coords: [1.0, 1.0], normal: [-1.0, 0.0, 0.0] }, 
            VertexPTN { position: [-0.5,  0.5,  0.5], tex_coords: [1.0, 0.0], normal: [-1.0, 0.0, 0.0] }, 
            VertexPTN { position: [-0.5,  0.5, -0.5], tex_coords: [0.0, 0.0], normal: [-1.0, 0.0, 0.0] }, 
        ];

        let indices: Vec<u32> = vec![
            0, 1, 2,  2, 3, 0,       // Front
            4, 5, 6,  6, 7, 4,       // Back
            8, 9, 10, 10, 11, 8,     // Top
            12, 13, 14, 14, 15, 12,  // Bottom
            16, 17, 18, 18, 19, 16,  // Right
            20, 21, 22, 22, 23, 20,  // Left
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Internal Cube Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Internal Cube Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        
        let mesh = MeshBuffers { vertex_buffer, index_buffer, num_indices: indices.len() as u32 };
        self.mesh_registry.insert(mesh_name.to_string(), MeshId(self.meshes.len()));
        self.meshes.push(mesh);
    }

    pub fn create_sphere_mesh(&mut self, mesh_name : &str,  device: &wgpu::Device, radius: f32, lat_bands: u32, lon_bands: u32){
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // 1. Vertices generieren (Breitengrade und Längengrade)
        for lat in 0..=lat_bands {
            let theta = (lat as f32 * PI) / lat_bands as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for lon in 0..=lon_bands {
                let phi = (lon as f32 * 2.0 * PI) / lon_bands as f32;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let nx = cos_phi * sin_theta;
                let ny = cos_theta;
                let nz = sin_phi * sin_theta;

                let u = lon as f32 / lon_bands as f32;
                let v = lat as f32 / lat_bands as f32;

                vertices.push(VertexPTN {
                    position: [nx * radius, ny * radius, nz * radius],
                    tex_coords: [u, v],
                    normal: [nx, ny, nz],
                });
            }
        }

        for lat in 0..lat_bands {
            for lon in 0..lon_bands {
                let first = lat * (lon_bands + 1) + lon;
                let second = first + lon_bands + 1;
                indices.push(first);
                indices.push(first + 1);
                indices.push(second);

                indices.push(first + 1);
                indices.push(second + 1);
                indices.push(second);
            }
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Internal Sphere Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Internal Sphere Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mesh = MeshBuffers { vertex_buffer, index_buffer, num_indices: indices.len() as u32 };
        self.mesh_registry.insert(mesh_name.to_string(), MeshId(self.meshes.len()));
        self.meshes.push(mesh);
    }
}


