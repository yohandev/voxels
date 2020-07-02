use std::collections::HashMap;

use ezgame::ecs::*;
use ezmath::*;

/// caches chunk using a hashmap
#[derive(Debug)]
pub struct RChunkCache
{
    map: HashMap<int3, Entity>,
}

impl RChunkCache
{
    /// create a new chunk cache resource
    pub(super) fn new() -> Self
    {
        Self { map: Default::default() }
    }

    /// stores a chunk in the cache. position
    /// is adjusted for you to the chunk grid
    pub fn store(&mut self, mut pos: int3, ent: Entity)
    {
        super::ChunkPos::adjust(&mut pos);

        self.map.insert(pos, ent);
    }

    /// remove a chunk from the cache. position
    /// is adjusted for you to the chunk grid
    pub fn release(&mut self, mut pos: int3)
    {
        super::ChunkPos::adjust(&mut pos);

        self.map.remove(&pos);
    }

    /// get the chunk where the block at pos is
    /// located. position is adjusted to chunk
    /// grid
    pub fn at(&self, mut pos: int3) -> Option<&Entity>
    {
        super::ChunkPos::adjust(&mut pos);

        self.map.get(&pos)
    }

    /// utility function to safely load a chunk
    /// and store it in the cache. nothing is done
    /// is the cache deems the chunk as already loaded.
    /// position is adjusted to chunk grid
    pub fn load(&mut self, cmd: &mut Cmd, mut pos: int3)
    {
        use super::*;

        // adjust pos
        ChunkPos::adjust(&mut pos);

        // ignore already loaded
        if self.map.contains_key(&pos)
        {
            return;
        }

        // chunk entity
        let ent = cmd.insert
        (
            (TUngenerated,),
            vec!
            [
                (CChunk::new(pos), CBlockBuffer::new())
            ],
        )[0];
        
        // store in cache
        self.store(pos, ent);
    }
}