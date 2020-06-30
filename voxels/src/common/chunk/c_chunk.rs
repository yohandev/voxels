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
    /// position of the block at the minimum corner
    /// in this chunk
    pub fn pos(&self) -> int3
    {
        self.pos
    }
}