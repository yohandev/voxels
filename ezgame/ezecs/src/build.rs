use std::collections::BinaryHeap;

/// `Systems` builder
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