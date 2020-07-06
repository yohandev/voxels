use crossbeam::queue::SegQueue;
use std::collections::HashMap;
use std::any::*;

use crate::*;
use super::*;

/// maps events to systems schedules.
#[derive(Default)]
pub struct EventSystems
{
    /// maps an event type to a vector of (priority, func ptr)'s
    handlers: HashMap<TypeId, Vec<(isize, EventHandler)>>,
    /// event queue of event types
    queue: SegQueue<TypeId>,
}

/// function ptr type that handles events
pub type EventHandler = fn(&mut crate::Application);

impl EventSystems
{
    /// inserts a system into the application. the event must be
    /// specified in the generic parameters because types can
    /// implement the `System` trait multiple times to handler more
    /// than one event.
    ///
    /// # example
    /// ```rust
    /// struct SPhysics;
    /// 
    /// impl System<StartEvent> for SPhysics
    /// {
    ///     fn run(app: &mut Application)
    ///     {
    ///         // initialize resources
    ///         app.resources().insert(RGravity { x: 0.0, y: -9.8, z: 0.0 });
    ///     }
    /// }
    /// 
    /// impl System<UpdateEvent> for SPhysics
    /// {
    ///     fn run(app: &mut Application)
    ///     {
    ///         if app.state().is::<PauseState>()
    ///         {
    ///             return
    ///         }
    ///         let registry = app.registry();
    ///         
    ///         let gravity = app.resources().get::<RGravity>();
    ///         let time = app.time();
    ///         
    ///         // acceleration -> velocity
    ///         for mut vel in <Write<CVel>>::query().iter_mut(registry)
    ///         {
    ///             vel -= gravity * time.dt();
    ///         }
    ///      
    ///         // velocity -> position
    ///         for (mut pos, vel) in <(Write<CPos>, Read<CVel>)>::query().iter_mut(registry)
    ///         {
    ///             pos += vel * time.dt();
    ///         }
    ///     }
    /// }
    /// ```
    /// the `SPhysics` struct implements the `System` trait twice,
    /// so the event must be specified at the `EventSystems::insert`
    /// call to prevent ambiguity
    pub fn insert<T: System<K>, K: 'static>(&mut self, sys: T)
    {
        let evt_id = TypeId::of::<K>();

        // first system for event
        if !self.handlers.contains_key(&evt_id)
        {
            // create new handler group
            self.handlers.insert(evt_id, Default::default());
        }
        // get all systems for event
        let group = self.handlers.get_mut(&evt_id).unwrap();
        
        // insert and sort
        group.push((T::ORDER, T::run));
        group.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    }

    /// recursively process all events pushed since the last
    /// process call.
    pub fn process(&mut self, app: &mut Application)
    {
        // retrieve events
        let events = std::mem::replace(&mut self.queue, Default::default());
        
        // break recursion
        if events.is_empty()
        {
            return;
        }

        // process all events
        while let Ok(evt) = events.pop()
        {
            if let Some(group) = self.handlers.get(&evt)
            {
                for (_, func) in group
                {
                    func(app);
                }
            }
        }
        
        // recurse
        self.process(app);
    }

    /// push an event to the event queue. the event won't be
    /// processed until all the preceding events have finished.
    ///
    /// this is the same as `Application::invoke`
    pub fn push_event<T: 'static>(&self)
    {
        self.queue.push(TypeId::of::<T>());
    }
}