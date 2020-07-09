use std::rc::Rc;

use ezmath::*;

use crate::common::block::{ PackedBlock, UnpackedBlock, RBlockPalette };
use crate::common::{ CHUNK_SIZE, CHUNK_LAYER, CHUNK_VOLUME };

/// component that stores a buffer of blocks,
/// typically associated with a CChunk component.
pub struct Chunk
{
    /// position of the min block in the chunk
    pos: int3,
    /// raw blocks storage
    blocks: Box<[PackedBlock; CHUNK_VOLUME]>,
    /// shared pointer to block palette, used
    /// for indexing without passing it in
    palette: Rc<RBlockPalette>,

    /// was the chunk generated yet?
    generated: bool,
    /// was the chunk updated during this frame?
    updated: bool,
}

/// trait that provides overridable methods for
/// indexing the CBlockBuffer component
pub trait ChunkIndex<T>
{
    /// get a packed block given a relative position,
    /// for low-level operations
    fn get_packed(&self, pos: T) -> PackedBlock;

    /// get an unpacked block given a relative position.
    fn get_unpacked(&self, pos: T) -> UnpackedBlock;

    /// set a packed given a relative position, giving
    /// direct access to the low-level chunk storage.
    /// this should only be used if you know what you're
    /// doing
    fn set_packed(&mut self, pos: T, val: PackedBlock);
}

impl Chunk
{
    /// create a new empty chunk, given the minimum chunk
    /// position. that position is automatically adjusted to
    /// snap to the 32x32x32 chunk grid.
    pub fn new(mut pos: int3, pal: &Rc<RBlockPalette>) -> Self
    {
        super::ChunkPos::adjust(&mut pos);

        Self
        {
            pos,
            blocks: Box::new([PackedBlock::default(); CHUNK_VOLUME]),
            palette: pal.clone(),

            generated: false,
            updated: false,
        }
    }

    /// position of the block at the minimum corner
    /// in this chunk
    pub fn pos(&self) -> int3
    {
        self.pos
    }

    /// has this chunk been generated yet?
    pub fn generated(&self) -> bool
    {
        self.generated
    }

    /// has this chunk been updated during this frame?
    pub fn updated(&self) -> bool
    {
        self.updated
    }

    /// mark this chunk as generated. this should only
    /// be called by the SChunkGen system
    pub(super) fn mark_generated(&mut self)
    {
        self.generated = true;
    }

    /// unmark this chunk as updated. this should only
    /// be called by the SChunkUpdated system
    pub(super) fn unmark_updated(&mut self)
    {
        self.updated = false;
    }
}

macro_rules! impl_index
{
    ($index_ty:ty, $x: tt, $y: tt, $z: tt) =>
    {
        impl ChunkIndex<$index_ty> for Chunk
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

            fn get_unpacked(&self, pos: $index_ty) -> UnpackedBlock
            {
                UnpackedBlock::new
                (
                    // packed
                    self.get_packed(pos),
                    // local pos
                    int3::new(pos.$x as i32, pos.$y as i32, pos.$z as i32),
                    // palette
                    Rc::as_ref(&self.palette)
                )
            }

            fn set_packed(&mut self, pos: $index_ty, val: PackedBlock)
            {
                let i = (pos.$x as usize              ) +
                        (pos.$y as usize * CHUNK_SIZE ) +
                        (pos.$z as usize * CHUNK_LAYER);
                
                if self.blocks[i] != val
                {
                    self.blocks[i]  = val;
                    self.updated    = true;
                }
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