use super::*;

const MAX_LOADED_CHUNKS: usize = 4096;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ChunkModel(pub [float3; MAX_LOADED_CHUNKS]);

impl ChunkModel
{
    pub fn create_uniform(ctx: &RenderCtx, slot: u32) -> Uniform<Self>
    {
        ctx.create_uniform()
            .with_binding_slot(slot)
            .with_visiblity(ShaderStage::VERTEX)
            .with_label("chunk_uniform")
            .build(Self::default())
    }
}

impl Default for ChunkModel
{
    fn default() -> Self
    {
        Self([float3::new(0.0, 0.0, 0.0); MAX_LOADED_CHUNKS])
    }
}

unsafe impl Pod for ChunkModel {}
unsafe impl Zeroable for ChunkModel {}
