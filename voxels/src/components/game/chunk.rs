use ezmath::*;

use crate::game::*;

/// chunk component
#[derive(Clone)]
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

/// tag to mark a chunk as currently loading
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkLoadTag;

/// tag for chunks that need generation. this shouldn't
/// be added directly.
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkGenerateTag;

impl Chunk
{
    /// create a new chunk at the given position
    pub fn new(pos: int3) -> Self
    {
        let x = pos.x - pos.x.rem_euclid(CHUNK_SIZE as i32);
        let y = pos.y - pos.y.rem_euclid(CHUNK_SIZE as i32);
        let z = pos.z - pos.z.rem_euclid(CHUNK_SIZE as i32);

        let blocks = Box::new([Block::default(); CHUNK_VOLUME]);
        let pos = int3::new(x, y, z);

        Self { blocks, pos }
    }

    /// get the position of the min block in this chunk
    ///
    /// if the chunk spans from (0, 0, 0) to (32, 32, 32),
    /// pos would be (0, 0, 0).
    pub fn position(&self) -> int3
    {
        self.pos
    }

    /// flatten a relative position index to a 1D array index
    fn flat_index(r_pos: &int3) -> usize
    {
        (r_pos.x as usize              ) +
        (r_pos.y as usize * CHUNK_SIZE ) +
        (r_pos.z as usize * CHUNK_LAYER)
    }

    /// flatten a relative position index to a 1D array index
    fn flat_index_tuple(r_pos: &(u32, u32, u32)) -> usize
    {
        (r_pos.0 as usize              ) +
        (r_pos.1 as usize * CHUNK_SIZE ) +
        (r_pos.2 as usize * CHUNK_LAYER)
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
        &mut self.blocks[Self::flat_index(&index)]
    }
}

impl Index<(u32, u32, u32)> for Chunk
{
    type Output = Block;

    /// get a block within this chunk, given a relative position
    fn index(&self, index: (u32, u32, u32)) -> &Self::Output
    {
        &self.blocks[Self::flat_index_tuple(&index)]
    }
}

impl IndexMut<(u32, u32, u32)> for Chunk
{
    /// get a block within this chunk, given a relative position
    fn index_mut(&mut self, index: (u32, u32, u32)) -> &mut Self::Output
    {
        &mut self.blocks[Self::flat_index_tuple(&index)]
    }
}