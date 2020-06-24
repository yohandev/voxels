use crate::*;

/// system function, which does the actual logic of
/// a system and is returned in the `System::exe`
/// function.
pub type SysFn = Box<dyn legion::prelude::Schedulable>;

/// a collection of systems, that may run in parallel.
/// basically, a legion`Schedule`. use the `Builder`
/// to create one of these, and not Systems::builder().
pub type Systems = legion::prelude::Schedule;

/// an event-responding system, that operates on entities
/// and their components. it's convention that systems
/// are prefixed with an `S`.
pub trait System: 'static
{
    /// event this system will respond to. the event
    /// itself can't have data directly, and should
    /// rather be fetched through resources.
    const EVENT: Event;
    /// the order in which this system is executed.
    /// if system `B` depends on system `A`, you
    /// should make `B::ORDER` equal to `A::ORDER + 1`.
    const ORDER: Order;

    /// should the system flush the command buffers
    /// before execution? this is so that this system
    /// can see entities and components added/removed
    /// by previous systems. ie: `SRender` system would
    /// most probably need this.
    const FLUSH: bool = false;

    /// before any system logic is run, insert resources
    /// inside the app so the game doesn't crash. this is
    /// optional, but should absolutely not run any logic!
    /// order within an event is guarenteed, but order of
    /// different evens is arbitrary.
    fn prepare(_: &mut Resources)
    {
        // ...
    }

    /// get the executing function of this system. use
    /// the `sys` function to implement this, and see
    /// the legion docs for more help.
    fn exe() -> SysFn;
}

/// creates a new system builder
pub fn sys(name: &'static str) -> SystemBuilder
{
    SystemBuilder::new(name)
}