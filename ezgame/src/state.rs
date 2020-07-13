use std::collections::HashMap;
use std::any::*;

use crate::ecs::*;
use super::*;

/// see `ezgame::StateMachine`
pub trait State: Any
{
    /// create a new state of Self type
    fn create(app: &mut Application) -> Self where Self: Sized;
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
    pub(super) states: HashMap<TypeId, (Box<dyn State>, Registry)>,
    pub(super) active: Option<TypeId>
}

impl Application
{
    /// switch to a state, or register it using its default.
    /// this will invoke the StateChange event if the state
    /// actually changed
    pub fn switch<T: State>(&mut self)
    {
        use std::any::*;

        if !self.state().is::<T>()
        {
            self.events().push::<evt::StateChanged>();

            let id = TypeId::of::<T>();

            // implicit register
            if !self.states.states.contains_key(&id)
            {
                let state = Box::new(T::create(self));
                let registry = self.create_registry();

                self.states.states.insert(id, (state, registry));
            }

            // switch
            self.states.active = Some(id);
        }
    }

    /// get the active state's registry. this is
    /// for systems that operate on *all* states
    /// but need access to entity registries
    /// (which are stored in the state).
    pub fn registry(&self) -> &Registry
    {
        &self.states.states
            .get(&self.states.active.unwrap())
            .unwrap()
            .1
    }

    /// get the active state's registry. this is
    /// for systems that operate on *all* states
    /// but need access to entity registries
    /// (which are stored in the state).
    pub fn registry_mut(&mut self) -> &mut Registry
    {
        &mut self.states.states
            .get_mut(&self.states.active.unwrap())
            .unwrap()
            .1
    }
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

    /// register a state without switching to it.
    /// the old one will be silently overwritten
    /// if it existed
    pub fn register<T: State>(&mut self, app: &mut Application)
    {
        self.states.insert
        (
            TypeId::of::<T>(),
            (Box::new(T::create(app)), app.create_registry())
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
            Some(self.states.get_mut(&id).unwrap().0.downcast_mut())
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
            Some(self.states.get(&id).unwrap().0.downcast_ref())
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
                Some((state, _)) => state.is::<T>(),
                None => false,
            },
            None => false,
        }
    }
}