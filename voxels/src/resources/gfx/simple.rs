use ezgame::plugins::ezgfx::*;
use ezmath::*;

/// resource storage required for simple rendering.
/// this is stored as an pption in the ECS world.
pub type SimpleGfxResources = Option<SimpleGfxResourcesStruct>;

/// use SimpleGfxResource instead.
pub struct SimpleGfxResourcesStruct
{
    pub vs: Shader,                         // shared vertex shader
    pub fs: Shader,                         // shared fragment shader

    pub geo: Geometry<SimpleVertex, u16>,   // shared geometry

    pub pipeline: Pipeline,                 // shared rendering pipeline

    pub vp: ViewProjBindGroup,              // shared view projection uniform
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

buffer_data!
(
    #[derive(Default)]
    pub struct ViewProjUniform
    {
        pub mat: float4x4
    }
);

type ViewProjBindGroup = BindGroup<(Uniform<ViewProjUniform>,)>;