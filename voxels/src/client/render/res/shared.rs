use ezgame::gfx::*;
use ezmath::*;

/// shared graphic resources
pub struct RGraphicsShared
{
    pub vp: ViewProjBindGroup,  //view projection uniform
}

buffer_data!
(
    #[derive(Default)]
    pub struct ViewProjUniform
    {
        pub mat: float4x4
    }
);

impl ViewProjUniform
{
    pub fn new(mat: float4x4) -> Self
    {
        Self { mat }
    }
}

type ViewProjBindGroup = BindGroup<(Uniform<ViewProjUniform>,)>;