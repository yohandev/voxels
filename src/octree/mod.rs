use crate::ezmath::*;

use stack::Stack;
use block::Block;
use node::Node;

mod block;
mod alloc;
mod stack;
mod node;
mod bits;

pub const DEPTH: usize = 8;

pub struct Octree
{
    heap: alloc::Allocator
}

pub struct Traversal
{
    stack: stack::Stack
}

impl Octree
{
    pub fn new(alloc: usize) -> Self
    {
        Self
        {
            heap: alloc::Allocator::new(alloc)
        }
    }

    pub fn get(&self, loc: float3) -> Traversal
    {
        let mut stack = Stack::default();
        let mut index = 0usize;

        let mut center = float3::new(0.0, 0.0, 0.0);
        let mut offset = float3::new(1.0, 1.0, 1.0) * (1 << (DEPTH - 2)) as f32;

        for d in 0..DEPTH
        {
            stack.push(index as u32);

            let bytes = &self.heap[index..(index + Node::SIZE)];

            let node = Node::from_bytes(bytes);     // current node
            
            if node.empty()                         // no children
            {
                break;
            }

            let dir = loc - center;                 // local direction

            offset.x = offset.x.copysign(dir.x);    // offset in correct direction
            offset.y = offset.y.copysign(dir.y);
            offset.z = offset.z.copysign(dir.z);

            if let Some(child) = node               // check child in direction
                .child_addr(&dir, d)
            {
                index = child as usize;
            }
            else                                    // no child in direction
            {
                break;
            }

            center += offset;
            offset *= 0.5;
        }

        Traversal { stack }
    }

    pub fn set(&mut self, loc: float3, block: Block)
    {
        
    }
}