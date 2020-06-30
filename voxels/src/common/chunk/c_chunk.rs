use ezmath::*;

/// chunk component
#[derive(Debug, Clone)]
pub struct CChunk
{
    /// position of the min block in the chunk
    pos: int3,
}

impl CChunk
{
    /// create a new chunk component, given the minimum chunk
    /// position. that position is automatically adjusted to
    /// snap to the 32x32x32 chunk grid.
    pub fn new(mut pos: int3) -> Self
    {
        use crate::common::CHUNK_SIZE;

        pos.x -= pos.x.rem_euclid(CHUNK_SIZE as i32);
        pos.y -= pos.y.rem_euclid(CHUNK_SIZE as i32);
        pos.z -= pos.z.rem_euclid(CHUNK_SIZE as i32);

        Self { pos }
    }

    /// position of the block at the minimum corner
    /// in this chunk
    pub fn pos(&self) -> int3
    {
        self.pos
    }
}