use super::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ChunkModel(pub float4x4);

impl ChunkModel
{
    pub fn create_uniform(ctx: &RenderCtx, slot: u32) -> Uniform<Self>
    {
        ctx.create_uniform()
            .with_binding_slot(slot)
            .with_visiblity(ShaderStage::VERTEX)
            .with_label("chunk_pos_uniform")
            .build(Self::default())
    }

    pub fn new(pos: &int3) -> Self
    {
        use nalgebra::*;

        let model = Translation3::new(pos.x as f32, pos.y as f32, pos.z as f32).to_homogeneous();
        
        Self(model)
    }
}

impl Default for ChunkModel
{
    fn default() -> Self
    {
        Self(float4x4::identity())
    }
}

unsafe impl Pod for ChunkModel {}
unsafe impl Zeroable for ChunkModel {}
