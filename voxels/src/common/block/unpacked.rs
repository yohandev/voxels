use ezmath::*;

use crate::common::chunk::ChunkPos;
use crate::common::CHUNK_SIZE;
use crate::common::block::*;

/// alias for unpacked block, which is relevant for
/// most usages
pub type Block<'a> = UnpackedBlock<'a>;

/// represents a block and its type data, holding
/// a temporary borrow to the game's block palette
pub struct UnpackedBlock<'a>
{
    packed:     PackedBlock,
    pos:        int3,

    pal: &'a    RBlockPalette,
}

#[allow(dead_code)]
impl<'a> UnpackedBlock<'a>
{
    // create a new unpacked block. this should not
    /// be called directly
    pub fn new(packed: PackedBlock, pos: int3, pal: &'a RBlockPalette) -> Self
    {
        Self { packed, pos, pal }
    }

    /// returns the packed version of self
    pub fn pack(&self) -> PackedBlock
    {
        self.packed
    }

    /// get the type ID of this block
    /// range is within 0..2048
    pub fn id(&self) -> usize
    {
        (match self.packed.format()
        {
            PackedBlockFormat::Data => self.packed.id(),
            PackedBlockFormat::Addr => todo!()
        })
        as usize
    }

    /// get the variant ID of this block
    /// range is within 0..16 and can represent
    /// orientation, texture changes, etc.
    pub fn variant(&self) -> usize
    {
        (match self.packed.format()
        {
            PackedBlockFormat::Data => self.packed.variant(),
            PackedBlockFormat::Addr => todo!()
        })
        as usize
    }

    /// get the textual representation of this block
    /// type's ID. this is typically snakecase
    pub fn text_id<'b>(&'b self) -> &'b str
    {
        self.pal.get(self.id()).id.as_str()
    }

    /// get the display name of this block
    pub fn name<'b>(&'b self) -> &'b str
    {
        self.pal.get(self.id()).name.as_str()
    }

    /// get the (debug) fallback color of this block,
    /// in case textures can't be loaded
    pub fn color(&self) -> float4
    {
        self.pal.get(self.id()).col
    }

    /// get the world position of this block
    pub fn pos(&self) -> int3
    {
        self.pos
    }

    /// get the relative(to chunk) position of this
    /// block
    pub fn r_pos(&self) -> int3
    {
        const SIZE: i32 = CHUNK_SIZE as i32;

        let rx = self.pos.x.rem_euclid(SIZE);
        let ry = self.pos.y.rem_euclid(SIZE);
        let rz = self.pos.z.rem_euclid(SIZE);

        int3::new(rx, ry, rz)
    }

    /// get the chunk position(min corner position)
    /// that this block is in
    pub fn c_pos(&self) -> int3
    {
        let mut pos = self.pos;

        ChunkPos::adjust(&mut pos);
        pos
    }

    /// get this block's shape(for rendering and collision)
    pub fn shape(&self) -> shapes::BlockShapes
    {
        self.pal.get(self.id()).shape
    }
}