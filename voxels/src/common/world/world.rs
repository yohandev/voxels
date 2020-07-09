use std::collections::hash_map::*;
use std::rc::Rc;

use ezmath::*;

use crate::common::block::*;
use super::*;

/// represents a dimension, which stores chunks
/// (and not living/block entities)
pub struct World
{
    map:        HashMap<int3, Chunk>,
    palette:    Rc<RBlockPalette>,
}

impl World
{
    pub(crate) fn new(palette: Rc<RBlockPalette>) -> Self
    {
        Self
        {
            map: Default::default(),
            palette,
        }
    }

    /// loads a chunk at a given position, doing nothing
    /// if already loaded
    pub fn load_chunk(&mut self, mut pos: int3)
    {
        // adjust to 32x32x32 grid
        ChunkPos::adjust(&mut pos);

        // prevent double-loading
        if !self.map.contains_key(&pos)
        {
            // load and store
            self.map.insert(pos, Chunk::new(pos, &self.palette));
        }
    }

    /// returns an immutable iterator over all the loaded
    /// chunks
    pub fn chunks(&self) -> Values<'_, int3, Chunk>
    {
        self.map.values()
    }

    /// returns a mutable iterator over all the loaded
    /// chunks
    pub fn chunks_mut(&mut self) -> ValuesMut<'_, int3, Chunk>
    {
        self.map.values_mut()
    }
}