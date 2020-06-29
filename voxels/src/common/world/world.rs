use std::collections::HashMap;
use ezmath::*;

use super::*;

/// world resource
pub type RWorld = World;

/// queries and contains loaded chunks
pub struct World
{
    /// read-only seed
    seed: u32,

    /// all currently loaded chunks
    chunks: HashMap<int3, Chunk>,

    /// dummy air block to return when falsy indexing
    air: Block,
}

impl World
{
    /// load a chunk given a block position. this
    /// function won't do anything if the chunk is
    /// already loaded.
    pub fn load(&mut self, mut pos: int3)
    {
        pos.x -= pos.x.rem_euclid(CHUNK_SIZE as i32);
        pos.y -= pos.y.rem_euclid(CHUNK_SIZE as i32);
        pos.z -= pos.z.rem_euclid(CHUNK_SIZE as i32);

        if !self.chunks.contains_key(&pos)
        {
            self.chunks.insert(pos, Chunk::new());
        }
    }

    /// get this world's seed
    pub fn seed(&self) -> u32
    {
        self.seed
    }
}