use std::ops::*;

use super::*;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_LAYER: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: usize = CHUNK_LAYER * CHUNK_SIZE;

pub struct Chunk
{
    /// raw blocks storage
    blocks: [Block; CHUNK_VOLUME],

    /// position of the min block in the chunk
    ///
    /// if the chunk spans from (0, 0, 0) to (32, 32, 32),
    /// pos would be (0, 0, 0).
    pos: int3
}

impl Chunk
{
    /// load a chunk
    pub fn load(world: &World, pos: int3) -> Self
    {
        let mut chunk = Self
        {
            blocks: [Block::default(); CHUNK_VOLUME],
            pos
        };
        generate(world, &mut chunk, &rand::thread_rng());

        chunk
    }

    /// flatten a relative position index to a 1D array index
    fn flat_index(r_pos: &int3) -> usize
    {
        (r_pos.x as usize              ) +
        (r_pos.y as usize * CHUNK_SIZE ) +
        (r_pos.z as usize * CHUNK_LAYER)
    }

    fn flat_index_tuple(r_pos: &(i32, i32, i32)) -> usize
    {
        (r_pos.0 as usize              ) +
        (r_pos.1 as usize * CHUNK_SIZE ) +
        (r_pos.2 as usize * CHUNK_LAYER)
    }

    pub fn pos(&self) -> &int3
    {
        &self.pos
    }

    pub fn x(&self) -> i32
    {
        self.pos.x
    }

    pub fn y(&self) -> i32
    {
        self.pos.y
    }

    pub fn z(&self) -> i32
    {
        self.pos.z
    }
}

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

impl Index<(i32, i32, i32)> for Chunk
{
    type Output = Block;

    /// get a block within this chunk, given a relative position
    fn index(&self, index: (i32, i32, i32)) -> &Self::Output
    {
        &self.blocks[Self::flat_index_tuple(&index)]
    }
}

impl IndexMut<(i32, i32, i32)> for Chunk
{
    /// get a block within this chunk, given a relative position
    fn index_mut(&mut self, index: (i32, i32, i32)) -> &mut Self::Output
    {
        &mut self.blocks[Self::flat_index_tuple(&index)]
    }
}