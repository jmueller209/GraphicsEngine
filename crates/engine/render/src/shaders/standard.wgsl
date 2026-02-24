// standard.wgsl

// --- Group 0: Global (Kamera) ---
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// --- Group 1: Material (Vom AssetManager) ---
@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

struct MaterialUniforms {
    roughness: f32,
    metallic: f32,
};
@group(1) @binding(2)
var<uniform> material: MaterialUniforms;

@group(1) @binding(3)
var t_normal: texture_2d<f32>;

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
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.world_normal = model.normal; // Für echte Normal-Maps bräuchtest du hier noch Tangenten!
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let diffuse_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let normal_map = textureSample(t_normal, s_diffuse, in.tex_coords);
    
    // Einfache Beleuchtung (Lambertian)
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.5));
    let sun_color = vec3<f32>(1.0, 0.9, 0.8);
    
    // Hier kombinieren wir die PBR-Werte aus der Uniform
    // In einem echten PBR-Shader würden roughness/metallic die Spiegelung beeinflussen
    let ambient = 0.1 * diffuse_color.rgb;
    let diff = max(dot(normalize(in.world_normal), light_dir), 0.0);
    
    // Mix aus Diffuse, Licht und Metallic-Look
    let final_rgb = ambient + (diffuse_color.rgb * sun_color * diff * (1.0 - material.roughness * 0.5));
    
    return vec4<f32>(final_rgb, diffuse_color.a);
}
