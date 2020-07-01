use ezmath::*;

use super::*;

/// stores data for a type of block. chunks store
/// "compressed" versions of these, which simply
/// point to the correct block data
#[derive(Debug)]
pub struct BlockData
{
    /// display name of the block
    display: String,
    /// id name of the block
    id_str: String,

    /// fallback colour of the block
    col: float4,

    /// block's shape
    shape: BlockShape,
}

/// block data resource. stores all types
/// of blocks in the game
#[derive(Debug)]
pub struct RBlockData
{
    blocks: Vec<BlockData>
}

impl RBlockData
{
    /// loads blocks given the path folder where
    /// their config is located
    pub fn load(_: &str) -> Self
    {
        Self { blocks: Default::default() }
    }

    /// get the display name of a block
    pub fn name<'a>(&'a self, b: Block) -> &'a str
    {
        self.blocks[b.id() as usize].display.as_str()
    }

    /// get the name identifier of a block,
    /// typically in snake case
    pub fn id<'a>(&'a self, b: Block) -> &'a str
    {
        self.blocks[b.id() as usize].id_str.as_str()
    }

    /// get the fallback colour of a block
    pub fn colour(&self, b: Block) -> float4
    {
        self.blocks[b.id() as usize].col
    }

    /// get the shape of a block
    pub fn shape(&self, b: Block) -> BlockShape
    {
        self.blocks[b.id() as usize].shape
    }
}