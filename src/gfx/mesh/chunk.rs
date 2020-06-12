use super::vertices::*;
use super::*;

pub struct ChunkMesh
{
    vertex_buf: Buffer,
    index_buf: Buffer,

    vertex_count: usize,
    index_count: usize,
}

impl ChunkMesh
{
    pub fn create(ctx: &RenderCtx, vert: &[ChunkVertex], ind: &[u32]) -> Self
    {
        let vertex_buf = ctx.create_buffer(vert, BufferUsage::VERTEX);
        let index_buf = ctx.create_buffer(ind, BufferUsage::INDEX);

        let vertex_count = vert.len();
        let index_count = ind.len();

        Self { vertex_buf, index_buf, vertex_count, index_count }
    }

    pub fn vertex_buffer(&self) -> &Buffer
    {
        &self.vertex_buf
    }

    pub fn index_buffer(&self) -> &Buffer
    {
        &self.index_buf
    }

    pub fn num_vertex(&self) -> usize
    {
        self.vertex_count
    }

    pub fn num_index(&self) -> usize
    {
        self.index_count
    }
}