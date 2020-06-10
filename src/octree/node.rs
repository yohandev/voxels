use crate::ezmath::*;
use super::alloc::*;
use super::bits::*;

pub(super) struct Node
{
    pub mask: u8,
    pub child: u32,
}

impl Node
{
    /// size, in bytes, of a node. not actual representation in memory
    pub const SIZE:usize = 5;

    /// parse a node from the heap, starting at index i
    pub(super) fn from_heap(heap: &Allocator, i: usize) -> Self
    {
        Self
        {
            mask: heap[i],
            child: u32::from_be_bytes
            ([
                heap[i + 1],
                heap[i + 2],
                heap[i + 3],
                heap[i + 4]
            ])
        }
    }

    /// does this node have no children(empty)
    pub(super) fn empty(&self) -> bool
    {
        self.mask == 0
    }

    /// get the index, n, of a node given the relative direction.
    /// direction doesn't need to be normalized.
    pub(super) fn child_index(dir: &float3) -> u8
    {
        let x = (dir.x >= 0.0) as u8;       // tests
        let y = (dir.y >= 0.0) as u8;
        let z = (dir.z >= 0.0) as u8;

        x | (y << 1) | (z << 2)             // child index, 0-7 (000 to 111)
    }

    /// get the index(address) on the virtual heap for the child
    /// in dir, relative to this node
    pub(super) fn child_addr(&self, dir: &float3, child_size: usize) -> Option<u32>
    {
        self.nth_child_addr(Self::child_index(dir), child_size)
    }

    /// get the index(address) on the virtual heap for the nth
    /// child of this node. returns none is the child doesn't exist.
    pub(super) fn nth_child_addr(&self, n: u8, child_size: usize) -> Option<u32>
    {        
        if self.mask & (1 << n) == 0        // no child in direction
        {
            None
        }
        else                                // found child
        {
            Some(self.child + (num_bits(self.mask & !(0xff << n)) * child_size as u32))
        }
    }

    /// range of memory occupied by this node's children
    pub(super) fn child_mem(&self) -> std::ops::Range<usize>
    {
        let lo = self.child as usize;
        let hi = lo + (num_bits(self.mask) as usize * Self::SIZE);

        std::ops::Range { start: lo, end: hi }
    }
}