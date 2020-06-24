use std::collections::BinaryHeap;

/// `Systems` builder
#[derive(Default)]
pub struct Builder
{
    sys: BinaryHeap<Sys>
}

/// temporary system structure that stores
/// a `System`'s `exe()` and `PRIORITY`.
struct Sys
{
    exe: crate::SysFn,
    ord: crate::Order,

    flush: bool,
}

impl Builder
{
    /// push a system, which is sorted by
    /// its order using the inner binary
    /// heap.
    pub fn push<T: crate::System>(&mut self)
    {
        self.sys.push(Sys
        {
            exe: T::exe(),
            ord: T::ORDER,
            flush: T::FLUSH,
        });
    }

    /// build the Systems, consuming the
    /// builder `self`.
    pub fn build(mut self) -> crate::Systems
    {
        let mut s = crate::Systems::builder();

        loop
        {
            if let Some(sys) = self.sys.pop()
            {
                if sys.flush
                {
                    s = s.flush();
                }
                s = s.add_system(sys.exe);
            }
            else { break; }
        }
        s.build()
    }
}

impl PartialEq for Sys
{
    fn eq(&self, other: &Self) -> bool
    {
        self.ord.eq(&other.ord)
    }
}

impl PartialOrd for Sys
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        self.ord.partial_cmp(&other.ord)
    }
}

impl Eq for Sys { }

impl Ord for Sys
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.ord.cmp(&other.ord)
    }
}