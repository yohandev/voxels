#version 450

layout(location = 0) in vec2 v_uv;

layout(location = 0) out vec4 f_color;

void main()
{
    f_color = vec4(v_uv.x, v_uv.y, 0.1, 1.0);
}