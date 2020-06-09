/// Represents a block, comprised(internally) of a single 2-byte integer.
/// | 1 bit | whether this block is model A or model B
///     - A: in place block data
///         | 11 bits | block ID, up to 2048 total blocks(probably more than enough)
///         | 4 bits  | block variant, useful for blocks like stairs(oriented) or wheat(grows)
///     - B: pointer block data
///         | 15 bits | pointer to big-block data, like chests or signs. points to chunk metadata
///
/// Model A is enough for *most* blocks, but others like chests or signs simply have too much data
/// to be inlined on the octree heap. Block B's pointers are 2^15, meaning chunks shouldn't exceed
/// 32x32x32 in the case that *all* blocks are model B
pub struct Block
{
    data: u16   // packed model(1 bit) + ID(11 bits) + variant(4 bits)
}

impl Block
{
    pub const SIZE:usize = std::mem::size_of::<Self>();
}