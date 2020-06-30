use ezmath::*;

use super::*;

/// number of blocks in one dimension of a chunk
/// currently, chunks are 32x32x32
pub const CHUNK_SIZE: usize = 32;
/// square of CHUNK_SIZE = 32 * 32
pub const CHUNK_LAYER: usize = CHUNK_SIZE * CHUNK_SIZE;
/// cube of CHUNK_SIZE = 32 * 32 * 32
/// total number of full blocks in a chunk
pub const CHUNK_VOLUME: usize = CHUNK_LAYER * CHUNK_SIZE;

/// a 32x32x32 portion of the world
pub struct Chunk
{
    /// raw blocks storage
    blocks: [Block; CHUNK_VOLUME],

    /// min corner position
    id: int3,

    /// current state
    state: ChunkState,
}

/// represents the state of a chunk for a given
/// frame
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChunkState
{
    /// chunk hasn't been generated.
    Loading,
    /// chunk has a block that's been updated.
    Updated,
    /// nothing happened to this chunk
    Nothing,
}

impl Chunk
{
    /// create a new chunk at the given position
    pub(super) fn new(id: int3) -> Self
    {
        Self
        {
            blocks: [Block::default(); CHUNK_VOLUME],
            id,
            state: ChunkState::Loading,
        }
    }

    /// flatten a relative position index to a 1D array index
    fn flat_index(r_pos: &int3) -> usize
    {
        (r_pos.x as usize              ) +
        (r_pos.y as usize * CHUNK_SIZE ) +
        (r_pos.z as usize * CHUNK_LAYER)
    }

    /// returns the current state of the chunk
    pub fn state(&self) -> ChunkState
    {
        self.state
    }

    /// get the position of the min corner in this
    /// chunk.
    pub fn id(&self) -> int3
    {
        self.id
    }
}

use std::ops::*;

impl Index<int3> for Chunk
{
    type Output = Block;

    /// get a block within this chunk, given a relative position
    fn index(&self, index: int3) -> &Self::Output
    {
        &self.blocks[Self::flat_index(&index)]
    }
}

impl IndexMut<int3> for Chunk
{
    /// get a block within this chunk, given a relative position
    fn index_mut(&mut self, index: int3) -> &mut Self::Output
    {
        self.state = ChunkState::Updated;
        &mut self.blocks[Self::flat_index(&index)]
    }
}