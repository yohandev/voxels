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
}