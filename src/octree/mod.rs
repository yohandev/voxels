use crate::ezmath::*;

use block::Block;
use node::Node;

mod block;
mod alloc;
mod node;
mod bits;

pub struct Octree
{
    depth: usize,
    heap: alloc::Allocator,
}

#[derive(Debug)]
pub struct Traversal
{
    stack: Vec<u32>
}

impl Octree
{
    pub fn new(depth: usize, alloc: usize) -> Self
    {
        Self
        {
            depth,
            heap: alloc::Allocator::new(alloc)
        }
    }

    pub fn get(&self, loc: float3) -> Traversal
    {
        let mut stack = Vec::with_capacity(self.depth);
        let mut index = 0usize;

        let mut center = float3::new(0.0, 0.0, 0.0);
        let mut offset = float3::new(1.0, 1.0, 1.0) * (1 << (self.depth - 2)) as f32;

        for d in 0..self.depth
        {
            stack.push(index as u32);

            let bytes = &self.heap[index..(index + Node::SIZE)];

            let node = Node::from_heap(&self.heap, index);                 // current node
            
            if node.empty()                         // no children
            {
                return Traversal { stack };
            }

            let dir = loc - center;                 // local direction

            offset.x = offset.x.copysign(dir.x);    // offset in correct direction
            offset.y = offset.y.copysign(dir.y);
            offset.z = offset.z.copysign(dir.z);

            let size = if d == self.depth - 1
            {
                Block::SIZE
            }
            else
            {
                Node::SIZE
            };

            if let Some(child) = node               // check child in direction
                .child_addr(&dir, size)
            {
                index = child as usize;
            }
            else                                    // no child in direction
            {
                return Traversal { stack };
            }

            center += offset;                       // center of next node
            offset *= 0.5;                          // decrease how offset of next iter
        }

        stack.push(index as u32);                   // index of found block

        Traversal { stack }
    }

    pub fn set(&mut self, loc: float3, block: Block)
    {
        let mut index = 0usize;

        let mut center = float3::new(0.0, 0.0, 0.0);
        let mut offset = float3::new(1.0, 1.0, 1.0) * (1 << (self.depth - 2)) as f32;

        for d in 0..self.depth
        {
            // 1. if node doesn't have child in direction:
            //      a. store previous mask(M1)
            //      b. set mask to *on* for direction
            //      c. de-alloc all current nodes, store them(N1)
            //      d. create a new array of size: N1.size + node size
            //      e. copy 0 to bits before index of dir in M1 range of N1 to new array
            //      f. add new node info that's being added in dir
            //      g. add the rest of the N1
            //      h. re-alloc the new array, set this node's first child index to whatever addr
            //      i. recurse down, new node = node just added
            // 2. else
            //      a. recurse down, new node = node found

            let node = Node::from_heap(&self.heap, index);                 // current node
            
            let dir = loc - center;                 // local direction

            offset.x = offset.x.copysign(dir.x);    // offset in correct direction
            offset.y = offset.y.copysign(dir.y);
            offset.z = offset.z.copysign(dir.z);

            let size = if d == self.depth - 1
            {
                Block::SIZE
            }
            else
            {
                Node::SIZE
            };

            let n = Node::child_index(&dir);

            if let Some(child) = node                   // check child in direction
                .nth_child_addr(n, size)
            {
                index = child as usize;
            }
            else                                        // no child in direction
            {
                let child_mem = node.child_mem();       // node's children mem

                let p_mask = node.mask;                 // previous mask
                let p_node =                            // previous children nodes in memory
                    &self.heap[child_mem.clone()]; 
                
                self.heap.free(child_mem);              // free previous nodes
                self.heap[0] = node.mask ^ (1u8 << n);  // update mask

                //let n_node = 
            }

            center += offset;                       // center of next node
            offset *= 0.5;                          // decrease how offset of next iter
        }
    }
}

#[test]
#[cfg(test)]
fn test_octree()
{
    let data = [0b00001000, 0, 0, 0, 5];

    let mut octree = Octree::new(2, 200);

    octree.heap.alloc(&[0b00001000]);
    octree.heap.alloc(&((octree.heap.len() + std::mem::size_of::<u32>()) as u32).to_be_bytes());
    octree.heap.alloc(&[0b00000101]);
    octree.heap.alloc(&((octree.heap.len() + std::mem::size_of::<u32>()) as u32).to_be_bytes());
    octree.heap.alloc(&Block::new(2).to_bytes());
    octree.heap.alloc(&Block::new(5).to_bytes());

    println!("{}", octree.heap);
    
    let found = octree.get(float3::new(0.0, 1.0, -2.0));

    println!("{:?}", found);
}