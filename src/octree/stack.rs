/// a mini stack, specific to this implementation of the SVO, that tracks the
/// recursion from root to leaf
pub(super) struct Stack
{
    data: [u32; super::DEPTH],
    next: usize,
}

impl Stack
{
    /// push an element to the end of the stack
    pub(super) fn push(&mut self, n: u32)
    {
        self.data[self.next] = n;
        self.next += 1;
    }

    /// retrieve the last element of the stack and remove it
    pub(super) fn pop(&mut self) -> u32
    {
        self.next -= 1;
        self.data[self.next]
    }
}

impl Default for Stack
{
    fn default() -> Self
    {
        Self
        {
            data: [0u32; super::DEPTH],
            next: 0
        }
    }  
}