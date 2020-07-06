use crate::*;

/// an event-responding system, that operates on entities
/// and their components. it's convention that systems
/// are prefixed with an `S`.
pub trait System<T>: 'static
{
    /// the order in which this system is executed.
    /// if system `B` depends on system `A`, you
    /// should make `B::ORDER` equal to `A::ORDER + 1`.
    const ORDER: isize;

    /// execute the system, receiving the event as an
    /// argument. 
    fn run(&mut self, app: &mut Application, evt: &T);
}