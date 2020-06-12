use super::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ModelViewProj(pub float4x4);

impl ModelViewProj
{
    pub fn create_uniform(ctx: &RenderCtx, slot: u32) -> Uniform<Self>
    {
        ctx.create_uniform_buffer()
            .with_binding_slot(slot)
            .with_visiblity(ShaderStage::VERTEX)
            .with_label("model_view_projection_uniform")
            .build(Self::default())
    }
}

impl Default for ModelViewProj
{
    fn default() -> Self
    {
        Self(float4x4::identity())
    }
}

unsafe impl Pod for ModelViewProj {}
unsafe impl Zeroable for ModelViewProj {}
