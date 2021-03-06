/// Represents a block, comprised(internally) of a single 2-byte integer.
/// | 1 bit | block format, data(A) or addr(B)
///     - A: in place block data
///         | 11 bits | block ID, up to 2048 total blocks(probably more than enough)
///         | 4 bits  | block variant, useful for blocks like stairs(oriented) or wheat(grows)
///     - B: pointer block data
///         | 15 bits | pointer to big-block data, like chests or signs. points to chunk metadata
///
/// Model A is enough for *most* blocks, but others like chests or signs simply have too much data
/// to be inlined on the block heap. Block B's pointers are 2^15, meaning chunks shouldn't exceed
/// 32x32x32 in the case that *all* blocks are model B
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct PackedBlock
{
    data: u16   // packed model(1 bit) + ID(11 bits) + variant(4 bits)
}

/// how block data is represented, either data or address
///     - data: all that needs to be known about the block is in the struct(grass, stairs, wheat, etc.)
///     - addr: the data in the struct points to larger block data stored in the chunk(chests, signs, etc.)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PackedBlockFormat { Data, Addr }

impl PackedBlock
{
    pub const SIZE:usize = std::mem::size_of::<Self>();

    pub fn new(data: u16) -> Self
    {
        Self { data }
    }

    /// get how this block is represented in memory
    pub fn format(self) -> PackedBlockFormat
    {
        match self.data >> 15
        {
            1 => PackedBlockFormat::Addr,
            _ => PackedBlockFormat::Data
        }
    }

    /// (unsafe) get this block's ID(0..2048) directly.
    ///
    /// data is innacurate if self.format() != BlockFormat::Data
    /// outright panics in debug mode
    pub fn id(self) -> u16
    {
        (self.data & 0b0111_1111_1111_0000) >> 4
    }

    /// (unsafe) get this block's variant(0..16) directly.
    ///
    /// data is innacurate if self.format() != BlockFormat::Data
    /// outright panics in debug mode
    pub fn variant(self) -> u16
    {
        self.data & 0b0000_0000_0000_1111
    }
}