use std::collections::HashMap;
use std::any::*;

use crate::ecs::*;

/// see `ezgame::StateMachine`
pub trait State: Any
{
    /// create a new state of Self type
    fn create() -> Self where Self: Sized;

    /// get an immutable list of this state's
    /// registries(if any). this is for systems
    /// that operate on *all* states but need
    /// access to entity registries(which are
    /// stored in the state)
    fn registries(&self) -> &[&Registry];

    /// get a mutable list of this state's
    /// registries(if any). this is for systems
    /// that operate on *all* states but need
    /// access to entity registries(which are
    /// stored in the state)
    fn registries_mut(&self) -> &[&mut Registry];
}

impl dyn State
{
    /// UNSAFE!!! downcasting that ignores the type
    /// check, because the state machine *knows* the
    /// type is right through the type map.
    fn downcast_ref<T: State>(&self) -> &T
    {
        unsafe { &*(self as *const dyn State as *const T) }
    }

    /// UNSAFE!!! downcasting that ignores the type
    /// check, because the state machine *knows* the
    /// type is right through the type map.
    fn downcast_mut<T: State>(&mut self) -> &mut T
    {
        unsafe { &mut *(self as *mut dyn State as *mut T) }
    }

    /// is this state the same type as the generic
    /// parameter T?
    pub fn is<T: State>(&self) -> bool
    {
        TypeId::of::<T>() == self.type_id()
    }
}

/// manages registered and active/unactive states,
/// where a state is arbitrary data that normally
/// shouldn't do or have much, other than being
/// a way for systems to differientiate game
/// stages and storing entity registries(if any)
pub struct StateMachine
{
    states: HashMap<TypeId, Box<dyn State>>,
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
    pub(super) fn switch<T: State>(&mut self)
    {
        let id = TypeId::of::<T>();

        // implicit register
        if !self.states.contains_key(&id)
        {
            self.states.insert(id, Box::new(T::create()));
        }

        // switch
        self.active = Some(id);
    }

    /// register a state without switching to it.
    /// the old one will be silently overwritten
    /// if it existed
    pub fn register<T: State>(&mut self)
    {
        self.states.insert
        (
            TypeId::of::<T>(),
            Box::new(T::create())
        );
    }

    /// attempts to cast the active state to the
    /// generic parameter. this won't panic, even if
    /// the generic paramater isn't the correct state,
    /// or the state hasn't been registered, or there
    /// is no active state at all.
    pub fn get_mut<T: State>(&mut self) -> Option<&mut T>
    {
        if let Some(id) = self.active
        {
            Some(self.states.get_mut(&id).unwrap().downcast_mut())
        }
        else
        {
            None
        }
    }

    /// attempts to cast the active state to the
    /// generic parameter. this won't panic, even if
    /// the generic paramater isn't the correct state,
    /// or the state hasn't been registered, or there
    /// is no active state at all.
    pub fn get<T: State>(&self) -> Option<&T>
    {
        if let Some(id) = self.active
        {
            Some(self.states.get(&id).unwrap().downcast_ref())
        }
        else
        {
            None
        }
    }

    /// checks whether the active state is
    /// of type T
    pub fn is<T: State>(&self) -> bool
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