use ezmath::*;

use crate::common::{ Block, CHUNK_SIZE, CHUNK_LAYER, CHUNK_VOLUME };

/// component that stores a buffer of blocks,
/// typically associated with a CChunk component.
#[derive(Clone)]
pub struct CBlockBuffer
{
    /// raw blocks storage
    blocks: Box<[Block; CHUNK_VOLUME]>,
}

impl CBlockBuffer
{
    /// create a new, empty, block buffer
    pub fn new() -> Self
    {
        Self
        {
            blocks: Box::new([Block::default(); CHUNK_VOLUME])
        }
    }
}

macro_rules! impl_index
{
    ($index_ty:ty, $x: tt, $y: tt, $z: tt) =>
    {
        impl std::ops::Index<$index_ty> for CBlockBuffer
        {
            type Output = Block;

            /// get a block within this chunk, given a relative position
            fn index(&self, index: $index_ty) -> &Self::Output
            {
                &self.blocks
                [
                    (index.$x as usize              ) +
                    (index.$y as usize * CHUNK_SIZE ) +
                    (index.$z as usize * CHUNK_LAYER)
                ]
            }
        }

        impl std::ops::IndexMut<$index_ty> for CBlockBuffer
        {
            /// get a block within this chunk, given a relative position
            fn index_mut(&mut self, index: $index_ty) -> &mut Self::Output
            {
                &mut self.blocks
                [
                    (index.$x as usize              ) +
                    (index.$y as usize * CHUNK_SIZE ) +
                    (index.$z as usize * CHUNK_LAYER)
                ]
            }
        }
    };
}

impl_index!(int3, x, y, z);
impl_index!(uint3, x, y, z);
impl_index!(isize3, x, y, z);
impl_index!(usize3, x, y, z);
impl_index!((i32, i32, i32), 0, 1, 2);
impl_index!((u32, u32, u32), 0, 1, 2);
impl_index!((isize, isize, isize), 0, 1, 2);
impl_index!((usize, usize, usize), 0, 1, 2);

impl_index!((isize, i32, i32), 0, 1, 2);
impl_index!((isize, isize, i32), 0, 1, 2);
impl_index!((isize, i32, isize), 0, 1, 2);
impl_index!((i32, isize, isize), 0, 1, 2);
impl_index!((i32, isize, i32), 0, 1, 2);

impl_index!((isize, u32, u32), 0, 1, 2);
impl_index!((isize, isize, u32), 0, 1, 2);
impl_index!((isize, u32, isize), 0, 1, 2);
impl_index!((u32, isize, isize), 0, 1, 2);
impl_index!((u32, isize, u32), 0, 1, 2);

impl_index!((usize, i32, i32), 0, 1, 2);
impl_index!((usize, usize, i32), 0, 1, 2);
impl_index!((usize, i32, usize), 0, 1, 2);
impl_index!((i32, usize, usize), 0, 1, 2);
impl_index!((i32, usize, i32), 0, 1, 2);

impl_index!((usize, u32, u32), 0, 1, 2);
impl_index!((usize, usize, u32), 0, 1, 2);
impl_index!((usize, u32, usize), 0, 1, 2);
impl_index!((u32, usize, usize), 0, 1, 2);
impl_index!((u32, usize, u32), 0, 1, 2);

impl_index!((i32, u32, u32), 0, 1, 2);
impl_index!((i32, i32, u32), 0, 1, 2);
impl_index!((i32, u32, i32), 0, 1, 2);
impl_index!((u32, i32, i32), 0, 1, 2);
impl_index!((u32, i32, u32), 0, 1, 2);