#version 450

layout(location = 0) in uint a_compressed;

layout(location = 0) out vec2 v_uv;

layout(set = 0, binding = 0) uniform ViewProjection
{
    mat4 u_view_proj;
};
layout(set = 1, binding = 0) uniform ChunkOffset
{
    ivec3 u_offset;
};

void main()
{
    float x = float(a_compressed >> 26) + float(u_offset.x);
    float y = float((a_compressed >> 20) & 63) + float(u_offset.y);
    float z = float((a_compressed >> 14) & 63) + float(u_offset.z);

    float u = float((a_compressed >> 7) & 127) / 128;
    float v = float(a_compressed & 127) / 128;

    v_uv = vec2(u, v) * float(gl_InstanceIndex) / 3;
    gl_Position = u_view_proj * vec4(x, y, z, 1.0);
}