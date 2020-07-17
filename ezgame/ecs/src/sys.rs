use crate::*;

/// an event-responding system, that operates on entities
/// and their components. it's convention that systems
/// are prefixed with an `S`.
pub trait System: 'static
{
    fn register(handlers: &mut Systems);
}

/// creates a new system builder
pub fn sys(name: &'static str) -> legion::prelude::SystemBuilder
{
    legion::prelude::SystemBuilder::new(name)
}