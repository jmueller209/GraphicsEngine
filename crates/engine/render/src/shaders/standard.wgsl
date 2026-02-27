// --- Group 0: Global (Kamera) ---
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// --- Group 1: Global Light Environment ---
struct LightInstance {
    position: vec3<f32>,
    light_type: u32,
    color: vec3<f32>,
    intensity: f32,
    direction: vec3<f32>,
    range: f32,
    cutoff: f32,
};

struct GlobalLightData {
    ambient_color: vec4<f32>, // rgb = Farbe, a = Intensität
    sun_direction: vec4<f32>, // xyz = Richtung, w = unused
    sun_color: vec4<f32>,     // rgb = Farbe, a = Intensität
    num_lights: u32,
    lights: array<LightInstance, 16>,
};

@group(1) @binding(0)
var<uniform> global_light: GlobalLightData;

// --- Group 2: Material ---
@group(2) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(2) @binding(1)
var s_diffuse: sampler;

struct MaterialUniforms {
    roughness: f32,
    metallic: f32,
};
@group(2) @binding(2)
var<uniform> material: MaterialUniforms;

@group(2) @binding(3)
var t_normal: texture_2d<f32>;

// --- NEU: Group 3: Model Matrizen (Storage Buffer) ---
struct ModelMatrixUniform {
    model: mat4x4<f32>,
};

@group(3) @binding(0)
var<storage, read> model_matrices: array<ModelMatrixUniform>;

// --- Vertex Input & Output ---
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) world_position: vec3<f32>, // Optional, nützlich für Lichtberechnungen
};

@vertex
fn vs_main(
    model: VertexInput,
    @builtin(instance_index) idx: u32 // <--- Den Index von Rust (draw_indexed) abgreifen
) -> VertexOutput {
    let model_data = model_matrices[idx]; // Die richtige Matrix für DIESES Objekt holen
    
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    
    // Normalen müssen auch transformiert werden (hier vereinfacht ohne Inverse-Transpose)
    // Wir nehmen nur den 3x3 Teil der Matrix für die Rotation
    out.world_normal = (model_data.model * vec4<f32>(model.normal, 0.0)).xyz;
    
    let world_pos = model_data.model * vec4<f32>(model.position, 1.0);
    out.world_position = world_pos.xyz;
    
    // Reihenfolge: Kamera * Modell * Vertex
    out.clip_position = camera.view_proj * world_pos;
    
    return out;
}



@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let diffuse_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let normal = normalize(in.world_normal);

    let ambient = global_light.ambient_color.rgb * global_light.ambient_color.a;
    let sun_dir = normalize(-global_light.sun_direction.xyz);
    let sun_diff_factor = max(dot(normal, sun_dir), 0.0);
    let sun_diffuse = global_light.sun_color.rgb * global_light.sun_color.a * sun_diff_factor;

    var point_light_color = vec3<f32>(0.0, 0.0, 0.0);
    
    for (var i = 0u; i < global_light.num_lights; i++) {
        let light = global_light.lights[i];
        
        let pixel_to_light = light.position - in.world_position;
        
        let distance = length(pixel_to_light);
        
        let light_dir = pixel_to_light / distance;
        
        let diff_factor = max(dot(normal, light_dir), 0.0);
        
        let attenuation = clamp(1.0 - (distance / light.range), 0.0, 1.0);
        
        point_light_color += light.color * light.intensity * diff_factor * attenuation;
    }

    var lighting = ambient + sun_diffuse + point_light_color;
    
    let final_rgb = diffuse_color.rgb * lighting * (1.0 - material.roughness * 0.3);
    
    return vec4<f32>(final_rgb, diffuse_color.a);
}
