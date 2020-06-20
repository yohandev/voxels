use anymap::AnyMap;

use crate::{ Entity, Component, Storage, EntAlloc };

/// a registry stores entities and their components.
#[derive(Debug)]
pub struct Registry
{
    entities: EntAlloc,
    components: AnyMap,
}

impl Registry
{
    /// create a new registry
    pub fn new() -> Self
    {
        Self
        {
            entities: EntAlloc::new(),
            components: AnyMap::new(),
        }
    }

    /// create a new entity
    pub fn create(&mut self) -> Entity
    {
        let ent = self.entities.alloc();

        ent
    }

    /// get the component storage for a component,
    /// if it exists
    pub fn storage<T: Component>(&self) -> Option<&T::Storage>
    {
        self.components.get::<T::Storage>()
    }

    /// get the component storage for a component,
    /// if it exists, but mutably
    pub fn storage_mut<T: Component>(&mut self) -> Option<&mut T::Storage>
    {
        self.components.get_mut::<T::Storage>()
    }

    /// get the component storage for a component,
    /// creating it if it doesn't exist.
    pub fn storage_or_create<T: Component>(&mut self) -> &mut T::Storage
    {
        if !self.components.contains::<T::Storage>()
        {
            self.components.insert(T::Storage::new());
        }
        self.components.get_mut::<T::Storage>().unwrap()
    }
}

/// trait that provides overloads for ezecs::Registry's
/// create() method
pub trait RegistryCreate<T>
{
    /// create an entity with the given components
    fn create(&mut self, cmp: T) -> Entity;
}

impl<T: Component> RegistryCreate<(T,)> for Registry
{
    fn create(&mut self, cmp: (T,)) -> Entity
    {
        // create entity
        let ent = self.entities.alloc();

        // init components
        self.storage_or_create::<T>().insert(ent, cmp.0);

        // return entity
        ent
    }
}