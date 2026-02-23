use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use image::GenericImageView;
use wgpu::util::DeviceExt;

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


// Structs for managing loaded assets
pub struct MeshBuffers {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}


#[derive(Debug, Clone)]
pub struct MaterialData {
    pub pipeline_name: String,
    pub bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct MaterialUniforms {
    roughness: f32,
    metallic: f32,
    _padding: [f32; 2],
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}


// impl Vertex for ModelVertex {
//     fn desc() -> wgpu::VertexBufferLayout<'static> {
//         use std::mem;
//         wgpu::VertexBufferLayout {
//             array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &[
//                 wgpu::VertexAttribute {
//                     offset: 0,
//                     shader_location: 0,
//                     format: wgpu::VertexFormat::Float32x3,
//                 },
//                 wgpu::VertexAttribute {
//                     offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
//                     shader_location: 1,
//                     format: wgpu::VertexFormat::Float32x2,
//                 },
//                 wgpu::VertexAttribute {
//                     offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
//                     shader_location: 2,
//                     format: wgpu::VertexFormat::Float32x3,
//                 },
//             ],
//         }
//     }
// }

pub struct AssetManager {
    pub default_sampler: wgpu::Sampler,
    pub default_normal_view: wgpu::TextureView,
    pub texture_views: HashMap<String, wgpu::TextureView>,
    pub meshes: HashMap<String, MeshBuffers>,
    pub materials: HashMap<String, MaterialData>,
    pub asset_manifest: AssetManifest,
    pub pipeline_cache: HashMap<String, wgpu::RenderPipeline>,
}

impl AssetManager {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let default_sampler = Self::create_default_sampler(device);
        let default_normal_view = Self::create_default_normal_view(device, queue);
        Self {
            default_sampler,
            default_normal_view,
            texture_views: HashMap::new(),
            meshes: HashMap::new(),
            materials: HashMap::new(),
            asset_manifest: AssetManifest {
                textures: HashMap::new(),
                meshes: HashMap::new(),
                materials: HashMap::new(),
            },
            pipeline_cache: HashMap::new(),
        }
    }

    pub fn load_manifest(&mut self, path: &str) {
        let file_content = std::fs::read_to_string(path)
            .expect("Error reading asset manifest file");
        self.asset_manifest = serde_json::from_str(&file_content)
            .expect("Error parsing asset manifest JSON");
    }

   pub fn initialize_assets(&mut self, manifest_path: &str, device: &wgpu::Device, queue: &wgpu::Queue) {
        let base_path = Path::new(manifest_path).parent().unwrap_or(Path::new(""));

        // Load textures
        for (name, relative_path) in &self.asset_manifest.textures {
            let mut full_path = PathBuf::from(base_path);
            full_path.push(relative_path);
            println!("Loading texture {:?} as {}.", full_path, name);
            let texture_view = self.load_texture_from_path(full_path, device, queue);
            self.texture_views.insert(name.clone(), texture_view);
        }
        
        // Load meshes
        for (name, relative_path) in &self.asset_manifest.meshes {
            let mut full_path = PathBuf::from(base_path);
            full_path.push(relative_path);
            println!("Loading mesh {:?} as {}.", full_path, name);
            let (vertex_buffer, index_buffer, num_indices) = self.load_mesh_from_path(full_path, device);
            self.meshes.insert(name.clone(), MeshBuffers {
                vertex_buffer,
                index_buffer,
                num_indices,
            });
        }

        // Create materials
        let materials = self.asset_manifest.materials.clone();
        for (name, config) in &materials {
            println!("Creating material {} with diffuse texture '{}'.", name, config.diffuse);
            let pipeline = self.pipeline_cache.get(&config.pipeline)
                .expect(&format!("Pipeline '{}' nicht im Cache gefunden. Lade sie, bevor initialize_assets aufgerufen wird!", config.pipeline));
            let layout = pipeline.get_bind_group_layout(0); 
            self.create_material(name, config, device, &layout);
        }

    }

    fn load_texture_from_path(&self, path: PathBuf, device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::TextureView {
        let img = image::open(&path).expect("Could not load texture image");
        
        // Wir prüfen wieder auf den Dateinamen für den Farbraum
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let is_data = file_name.contains("_n.") || file_name.contains("_data.");
        
        let format = if is_data {
            wgpu::TextureFormat::Rgba8Unorm
        } else {
            wgpu::TextureFormat::Rgba8UnormSrgb
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

    fn load_mesh_from_path(&self, path: PathBuf, device: &wgpu::Device) -> (wgpu::Buffer, wgpu::Buffer, u32) {
        let (models, _materials) = tobj::load_obj(
            &path,
            &tobj::LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        ).expect(&format!("Fehler beim Laden der OBJ-Datei: {:?}", path));

        let m = &models[0];

        let vertices = (0..m.mesh.positions.len() / 3)
            .map(|i| {
                ModelVertex {
                    position: [
                        m.mesh.positions[i * 3],
                        m.mesh.positions[i * 3 + 1],
                        m.mesh.positions[i * 3 + 2],
                    ],
                    tex_coords: [
                        m.mesh.texcoords[i * 2], 
                        1.0 - m.mesh.texcoords[i * 2 + 1]
                    ],
                    normal: if m.mesh.normals.is_empty() {
                        [0.0, 0.0, 0.0]
                    } else {
                        [
                            m.mesh.normals[i * 3],
                            m.mesh.normals[i * 3 + 1],
                            m.mesh.normals[i * 3 + 2],
                        ]
                    },
                }
            })
            .collect::<Vec<_>>();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Vertex Buffer", path)),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Index Buffer", path)),
            contents: bytemuck::cast_slice(&m.mesh.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        (vertex_buffer, index_buffer, m.mesh.indices.len() as u32)
    }

    pub fn create_material(
        &mut self,
        name: &str,
        config: &MaterialConfig,
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
    ) {
        let diffuse_view = self.texture_views.get(&config.diffuse).expect("Texture not found");
        
        let normal_view = if let Some(normal_name) = &config.normal {
            self.texture_views.get(normal_name).expect("Normal Texture not found")
        } else {
            &self.default_normal_view
        };

        let uniforms = MaterialUniforms {
            roughness: config.roughness,
            metallic: config.metallic,
            _padding: [0.0; 2],
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Uniform Buffer", name)),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: wgpu::BindingResource::TextureView(diffuse_view) },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::Sampler(&self.default_sampler) },
                wgpu::BindGroupEntry { binding: 2, resource: uniform_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(normal_view)},
            ],
            label: Some(&format!("Material BindGroup: {}", name)),
        });
        self.materials.insert(
            name.to_string(),
            MaterialData {
                pipeline_name: config.pipeline.clone(),
                bind_group,
            },
        );
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

    fn create_default_normal_view(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::TextureView {
        let size = wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Default Normal Map"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
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
            &[128, 128, 255, 255],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            size,
        );

        texture.create_view(&wgpu::TextureViewDescriptor::default())
    }
}


