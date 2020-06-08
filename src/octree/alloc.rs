// maximum size of bytes that can be allocated or freed to the allocator.
// since the implementation is very specific and allocations size never exceed
// ~40 bytes, this option is viable and merging isn't even necessary. the array
// is partially populated with vectors, populated only with at indices that are
// multiples(1-8) of DATA_SIZE and NODE_SIZE.
const MAX_ALLOC: usize = max(super::DATA_SIZE, super::NODE_SIZE) * 8;

/// virtual allocator that works over its own subset of contiguous real memory
/// implementation is very specific to this SVO implementation
pub(super) struct Allocator
{
    mem: Vec<u8>,                           // contiguous memory this allocator manages
    free: Vec<Option<Vec<usize>>>           // free blocks of memory per specific size.
}

impl Allocator
{
    pub(super) fn new(capacity: usize) -> Self
    {
        Self
        {
            mem: Vec::with_capacity(capacity),
            free: Vec::with_capacity(MAX_ALLOC)
        }
    }

    /// allocate a set number of bytes, <= MAX_ALLOC and populate it with the
    /// provided data. returns the address(index) of the start of the allocated
    /// data.
    pub(super) fn alloc(&mut self, data: &[u8]) -> usize
    {
        let len = data.len();

        if self.free[len].is_none()
        {
            self.free[len] = Some(Vec::default());
            self.alloc(data)
        }
        else if let Some(free) = self.free[len]
            .as_mut()
            .unwrap()
            .pop()
        {
            for (i, byte) in data.iter().enumerate()
            {
                self.mem[free + i] = *byte;
            }
            free
        }
        else
        {
            self.mem.extend_from_slice(data);
            self.mem.len() - len
        }
    }

    /// mark the len amount of bytes starting at loc as free, to be re-allocated
    /// using the alloc() function.
    pub(super) fn free(&mut self, loc: usize, len: usize)
    {
        if self.free[len].is_none()
        {
            self.free[len] = Some(Vec::default());
        }
        self.free[len]
            .as_mut()
            .unwrap()
            .push(loc);
    }
}

/// from https://stackoverflow.com/a/53646925
const fn max(a: usize, b: usize) -> usize
{
    [a, b][(a < b) as usize]
}