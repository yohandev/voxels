use ezgame::plugins::ezgfx::*;
use ezmath::*;

/// resource storage required for simple rendering.
/// this is stored as an Option<SimpleGfxResources> in
/// the ECS world.
pub struct SimpleGfxResources
{
    pub vs: Shader,                         // shared vertex shader
    pub fs: Shader,                         // shared fragment shader

    pub geo: Geometry<SimpleVertex, u16>,   // shared geometry

    pub pipeline: Pipeline,                 // shared rendering pipeline
}

buffer_data!
(
    pub struct SimpleVertex
    {
        pub pos: float3,
        pub col: float3,
    }
);

impl Vertex for SimpleVertex
{
    const DESC: &'static [VertexAttr] = &[VertexAttr::Float3, VertexAttr::Float3];
}