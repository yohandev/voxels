use crate::ezmath::*;
use super::bits::*;

pub(super) struct Node
{
    mask: u8,
    child: u32,
}

impl Node
{
    /// size, in bytes, of a node. not actual representation in memory
    pub const SIZE:usize = 5;

    /// does this node have no children(empty)
    pub(super) fn empty(&self) -> bool
    {
        self.mask == 0
    }

    /// get the index(address) on the virtual heap for the child
    /// in dir, relative to this node
    pub(super) fn child_addr(&self, dir: &float3, depth: usize) -> Option<u32>
    {
        let x = (dir.x >= 0.0) as u32;      // tests
        let y = (dir.y >= 0.0) as u32;
        let z = (dir.z >= 0.0) as u32;

        let n = x | (y << 1) | (z << 2);    // child index, 0-7 (000 to 111)

        self.nth_child_addr(n as u8, depth)
    }

    /// get the index(address) on the virtual heap for the nth
    /// child of this node. returns none is the child doesn't exist.
    fn nth_child_addr(&self, n: u8, depth: usize) -> Option<u32>
    {
        let size = if depth == super::DEPTH - 1 { super::block::Block::SIZE } else { super::node::Node::SIZE };

        if self.mask & (1 << n) == 0        // no child in direction
        {
            None
        }
        else                                // found child
        {
            Some(self.child + (num_bits(self.mask & !(0xff << n)) * size as u32))
        }
    }

    pub(super) fn from_bytes(bytes: &[u8]) -> Self
    {
        Self
        {
            mask: bytes[0],
            child: u32::from_le_bytes([bytes[1], bytes[2], bytes[3], bytes[4]])
        }
    }
}