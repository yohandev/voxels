use crossbeam::queue::SegQueue;
use std::any::TypeId;

/// event queue resource, at the core of ezgame
/// applications. it supports multi-threaded event
/// invoking and processes them recursively. apart
/// from the `push` method, this resource shouldn't 
/// be tempered with.
pub struct REventQueue
{
    pub(super) queue: SegQueue<TypeId>
}

impl REventQueue
{
    pub(crate) fn new() -> Self
    {
        Self { queue: Default::default() }
    }

    /// queue an event to be processed later
    pub fn push<T: 'static>(&self)
    {
        self.queue.push(TypeId::of::<T>());
    }
}