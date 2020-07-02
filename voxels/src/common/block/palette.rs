use ezmath::*;

use shapes::*;
use super::*;

/// stores data for a type of block. chunks store
/// "compressed" versions of these, which simply
/// point to the correct block data
#[derive(Debug)]
pub struct BlockData
{
    /// display name of the block
    pub name: String,
    /// id name of the block
    pub id: String,

    /// fallback colour of the block
    pub col: float4,

    /// block's shape
    pub shape: BlockShapes,
}

/// block data resource. stores all types
/// of blocks in the game
#[derive(Debug)]
pub struct RBlockPalette
{
    blocks: Vec<BlockData>
}

impl RBlockPalette
{
    /// loads blocks given the path folder where
    /// their config is located
    pub fn load(_: &str) -> Self
    {
        Self { blocks: Default::default() }
    }

    /// get block data for a given block ID
    pub(super) fn get<'a>(&'a self, id: usize) -> &'a BlockData
    {
        &self.blocks[id]
    }
}