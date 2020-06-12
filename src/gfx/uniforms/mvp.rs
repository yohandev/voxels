use super::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
/// a camera's view-projection matrix
pub struct ViewProj(pub float4x4);

impl ViewProj
{
    pub fn create_uniform(ctx: &RenderCtx, slot: u32) -> Uniform<Self>
    {
        ctx.create_uniform()
            .with_binding_slot(slot)
            .with_visiblity(ShaderStage::VERTEX)
            .with_label("model_view_projection_uniform")
            .build(Self::default())
    }
}

impl Default for ViewProj
{
    fn default() -> Self
    {
        Self(float4x4::identity())
    }
}

unsafe impl Pod for ViewProj {}
unsafe impl Zeroable for ViewProj {}
