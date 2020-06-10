// maximum size of bytes that can be allocated or freed to the allocator.
// since the implementation is very specific and allocations size never exceed
// ~40 bytes, this option is viable and merging isn't even necessary. the array
// is partially populated with vectors, populated only with at indices that are
// multiples(1-8) of DATA_SIZE and NODE_SIZE.
const MAX_ALLOC: usize = max(super::block::Block::SIZE, super::node::Node::SIZE) * 8;

/// virtual allocator that works over its own subset of contiguous real memory
/// implementation is very specific to this SVO implementation
#[derive(Debug)]
pub(super) struct Allocator
{
    pub(super) mem: Vec<u8>,                // contiguous memory this allocator manages

    free: Vec<Option<Vec<usize>>>           // free blocks of memory per specific size.
}

impl Allocator
{
    pub(super) fn new(capacity: usize) -> Self
    {
        let mut free = Vec::with_capacity(MAX_ALLOC);

        free.resize(MAX_ALLOC, None);

        Self
        {
            mem: Vec::with_capacity(capacity),
            free
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
    pub(super) fn free(&mut self, range: std::ops::Range<usize>)
    {
        let loc = range.start;
        let len = range.end - range.start;

        if self.free[len].is_none()
        {
            self.free[len] = Some(Vec::default());
        }
        self.free[len]
            .as_mut()
            .unwrap()
            .push(loc);
    }

    pub(super) fn len(&self) -> usize
    {
        self.mem.len()
    }
}

impl std::ops::Index<usize> for Allocator
{
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output
    {
        &self.mem[index]
    }
}

impl std::ops::IndexMut<usize> for Allocator
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        &mut self.mem[index]
    }
}

impl std::ops::Index<std::ops::Range<usize>> for Allocator
{
    type Output = [u8];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output
    {
        &self.mem[index]
    }
}

impl std::ops::IndexMut<std::ops::Range<usize>> for Allocator
{
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut Self::Output
    {
        &mut self.mem[index]
    }
}

impl std::fmt::Display for Allocator
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let mut str_repr = self.mem
            .iter()
            .map(|b| format!("[{:03}]", b))
            .collect::<Vec<String>>();

        for (len, locs) in self.free
            .iter()
            .enumerate()
            .filter(|(_, locs)| locs.is_some())
            .map(|(i, e)| (i, e.as_ref().unwrap()))
        {
            for loc in locs
            {
                for i in *loc..(loc + len)
                {
                    str_repr[i] = String::from("[###]");
                }
            }
        }

        f.write_str(str_repr.concat().as_str())
    }
}

/// from https://stackoverflow.com/a/53646925
const fn max(a: usize, b: usize) -> usize
{
    [a, b][(a < b) as usize]
}