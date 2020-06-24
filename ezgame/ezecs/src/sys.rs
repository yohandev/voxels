use crate::*;

/// system function, which does the actual logic of
/// a system and is returned in the `System::get_fn`
/// function.
pub type SysFn = Box<dyn legion::prelude::Schedulable>;

/// an event-responding system, that operates on entities
/// and their components.
pub trait System
{
    /// event this system will respond to. the event
    /// itself can't have data directly, and should
    /// rather be fetched through resources.
    const EVENT: Event;
    /// the order in which this system is executed.
    /// if system `B` depends on system `A`, you
    /// should make `B::ORDER` equal to `A::ORDER + 1`.
    const ORDER: Order;

    /// get the executing function of this system. use
    /// the `sys` function to implement this, and see
    /// the legion docs for more help.
    fn get_fn() -> SysFn;
}