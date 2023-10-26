
// Vertex shader

struct VertexInput
{
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
    @location(1) color: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    in: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.color = in.position.xyx;
    out.tex_coord = in.uv;
    return out;
}




// Fragment shader

@group(0) @binding(0)
var t_diff: texture_2d<f32>;

@group(0) @binding(1)
var s_diff: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diff, s_diff, in.tex_coord);//vec4<f32>(in.color, 1.0);
}