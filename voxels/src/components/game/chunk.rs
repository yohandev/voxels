use ezmath::*;

use crate::game::*;

/// chunk component
pub struct Chunk
{
    /// raw blocks storage
    blocks: Box<[Block; CHUNK_VOLUME]>,

    /// position of the min block in the chunk
    ///
    /// if the chunk spans from (0, 0, 0) to (32, 32, 32),
    /// pos would be (0, 0, 0).
    pos: int3,
}