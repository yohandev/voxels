use ezmath::*;

/// chunk position utility functions. chunk positions
/// are int3's
pub struct ChunkPos;

impl ChunkPos
{
    /// adjust an arbitrary world position
    /// to a chunk position
    pub fn adjust(pos: &mut int3)
    {
        const SIZE: i32 = crate::common::CHUNK_SIZE as i32;

        pos.x -= pos.x.rem_euclid(SIZE);
        pos.y -= pos.y.rem_euclid(SIZE);
        pos.z -= pos.z.rem_euclid(SIZE);
    }
}