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
    out.world_normal = model.normal;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let diffuse_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    let normal = normalize(in.world_normal);

    // 1. Ambient Light
    // Wir nehmen die Farbe und skalieren sie mit der Intensität (alpha)
    let ambient = global_light.ambient_color.rgb * global_light.ambient_color.a;
    
    // 2. Sun Light (Directional)
    // Die sun_direction zeigt VOM Licht weg, wir brauchen den Vektor ZUM Licht
    let light_dir = normalize(-global_light.sun_direction.xyz);
    let diff_factor = max(dot(normal, light_dir), 0.0);
    let sun_diffuse = global_light.sun_color.rgb * global_light.sun_color.a * diff_factor;

    // 3. Kombination der Lichtquellen
    // Einfaches Lambert-Modell: (Ambient + Sonne) * Textur
    var lighting = ambient + sun_diffuse;
    
    // Kleiner Bonus: Roughness beeinflusst, wie stark das diffuse Licht reflektiert wird
    let final_rgb = diffuse_color.rgb * lighting * (1.0 - material.roughness * 0.3);
    
    return vec4<f32>(final_rgb, diffuse_color.a);
}
