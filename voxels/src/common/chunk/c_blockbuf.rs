use ezmath::*;

use crate::common::block::{ PackedBlock, UnpackedBlock, RBlockPalette };
use crate::common::{ CHUNK_SIZE, CHUNK_LAYER, CHUNK_VOLUME };

/// component that stores a buffer of blocks,
/// typically associated with a CChunk component.
#[derive(Clone)]
pub struct CBlockBuffer
{
    /// raw blocks storage
    blocks: Box<[PackedBlock; CHUNK_VOLUME]>,
}

/// trait that provides overridable methods for
/// indexing the CBlockBuffer component
pub trait BlockBufferIndex<T>
{
    /// get a packed block given a relative position,
    /// for low-level operations
    fn get_packed(&self, pos: T) -> PackedBlock;

    /// get an unpacked block given a relative position
    fn get<'a>(&'a self, pos: T, pal: &'a RBlockPalette) -> UnpackedBlock;

    /// set a packed given a relative position, giving
    /// direct access to the low-level chunk storage.
    /// this should only be used if you know what you're
    /// doing
    fn set_packed(&mut self, pos: T, val: PackedBlock);
}

impl CBlockBuffer
{
    /// create a new, empty, block buffer
    pub fn new() -> Self
    {
        Self
        {
            blocks: Box::new([PackedBlock::default(); CHUNK_VOLUME])
        }
    }
}

macro_rules! impl_index
{
    ($index_ty:ty, $x: tt, $y: tt, $z: tt) =>
    {
        impl BlockBufferIndex<$index_ty> for CBlockBuffer
        {
            fn get_packed(&self, pos: $index_ty) -> PackedBlock
            {
                self.blocks
                [
                    (pos.$x as usize              ) +
                    (pos.$y as usize * CHUNK_SIZE ) +
                    (pos.$z as usize * CHUNK_LAYER)
                ]
            }

            fn get<'a>(&'a self, pos: $index_ty, pal: &'a RBlockPalette) -> UnpackedBlock
            {
                UnpackedBlock::new
                (
                    // packed
                    self.get_packed(pos),
                    // local pos
                    int3::new(pos.$x as i32, pos.$y as i32, pos.$z as i32),
                    // palette
                    pal
                )
            }

            fn set_packed(&mut self, pos: $index_ty, val: PackedBlock)
            {
                self.blocks
                [
                    (pos.$x as usize              ) +
                    (pos.$y as usize * CHUNK_SIZE ) +
                    (pos.$z as usize * CHUNK_LAYER)
                ] = val;
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