struct Uniforms {
    color_rgb: vec4<f32>,
}

@group(1) @binding(0)
var<uniform> uniforms: Uniforms;
