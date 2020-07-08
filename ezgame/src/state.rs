use std::collections::HashMap;
use std::any::*;

/// manages registered and active/unactive states,
/// where a state is arbitrary data that normally
/// shouldn't do or have much, other than being
/// a way for systems to differientiate game
/// stages and storing entity registries(if any)
pub struct StateMachine
{
    states: HashMap<TypeId, Box<dyn Any>>,
    active: Option<TypeId>
}

impl StateMachine
{
    pub(crate) fn new() -> Self
    {
        Self
        {
            states: Default::default(),
            active: Default::default(),
        }
    }

    /// switch to a state, or register it using its default
    pub fn switch<T: Any + Default>(&mut self)
    {
        let id = TypeId::of::<T>();

        // implicit register
        if !self.states.contains_key(&id)
        {
            self.states.insert(id, Box::new(T::default()));
        }

        // switch
        self.active = Some(id);
    }

    /// register a state without switching to it.
    /// the old one will be silently overwritten
    /// if it existed
    pub fn register<T: Any + Default>(&mut self)
    {
        self.states.insert
        (
            TypeId::of::<T>(),
            Box::new(T::default())
        );
    }

    /// attempts to cast the active state to the
    /// generic parameter. this won't panic, even if
    /// the generic paramater isn't the correct state,
    /// or the state hasn't been registered, or there
    /// is no active state at all.
    pub fn active<T: Any + Default>(&mut self) -> Option<&mut T>
    {
        if let Some(id) = self.active
        {
            self.states.get_mut(&id).unwrap().downcast_mut()
        }
        else
        {
            None
        }
    }

    /// checks whether the active state is
    /// of type T
    pub fn is<T: Any + Default>(&self) -> bool
    {
        match self.active
        {
            Some(id) => match self.states.get(&id)
            {
                Some(state) => state.is::<T>(),
                None => false,
            },
            None => false,
        }
    }
}