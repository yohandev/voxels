use crate::{ Entity, Component };

mod dense;
pub use dense::DenseStorage;

pub trait Storage<T: Component>
{
    /// create a new storage
    fn new() -> Self;

    /// insert a component for an entity into
    /// the storage. old component will be
    /// silently overwritten if already present.
    fn insert(&mut self, ent: Entity, cmp: T);

    /// remove a component from an entity in this
    /// storage. returns
    fn remove(&mut self, ent: Entity) -> Option<T>;
}