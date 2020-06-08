use crate::ezmath::*;

mod alloc;
mod stack;
mod bits;

pub const DEPTH: usize = 8;

pub const DATA_SIZE: usize = 2;
pub const NODE_SIZE: usize = 5;

pub struct Octree
{
    mem: alloc::Allocator
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
            mem: alloc::Allocator::new(alloc)
        }
    }

    pub fn at(&self, loc: float3) -> Traversal
    {
        let mut stack = stack::Stack::default();
        let mut index = 0;

        let mut center = float3::new(0.0, 0.0, 0.0);
        let mut offset = float3::new(1.0, 1.0, 1.0) * (1 << (DEPTH - 2)) as f32;

        for d in 0..DEPTH
        {
            stack.push(index);

            let mask = self.mem[index as usize];    // child mask
            
            if mask == 0                            // no children
            {
                break;
            }

            let dir = loc - center;                 // local direction

            let x = (dir.x >= 0.0) as u32;
            let y = (dir.y >= 0.0) as u32;
            let z = (dir.z >= 0.0) as u32;

            offset.x = offset.x.copysign(dir.x);
            offset.y = offset.y.copysign(dir.y);
            offset.z = offset.z.copysign(dir.z);

            let child = x | (y << 1) | (z << 2);    // child index, 0-7 (000 to 111)

            if mask & (1 << child) == 0             // no child in direction
            {
                break;
            }

            use std::convert::TryInto;

            index =
            {
                let first_child = u32::from_ne_bytes
                (
                    self.mem.mem[(index + 1) as usize..(index + 5) as usize]
                        .try_into()
                        .unwrap()
                );
                if d == DEPTH - 1                   // next depth is leaf
                {
                    first_child + (child * DATA_SIZE as u32)
                }
                else                                // next depth is branch
                {
                    first_child + (child * NODE_SIZE as u32)
                }
            };

            center += offset;
            offset *= 0.5;
        }

        Traversal { stack }
    }
}