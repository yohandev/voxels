use std::collections::HashMap;
use std::any::TypeId;

use crate::*;

/// an event-responding system, that operates on entities
/// and their components. it's convention that systems
/// are prefixed with an `S`. all the implementation of
/// the `Systems` trait does is the `register` method, where
/// you bind regular `impl` block methods to events.
///
/// # example
/// ```rust
/// struct SPhysics;
/// 
/// impl System for SPhysics
/// {
///     fn register(handlers: Systems)
///     {
///         handlers.insert::<evt::Start>(0, Self::on_start);
///         handlers.insert::<evt::Update>(-999, Self::on_update);
///         handlers.insert::<evt::Quit>(999, Self::on_quit);
///     }
/// }
///
/// impl System
/// {
///     // system functions must have this signature
///     fn on_start(app: &mut Application)
///     {
///         app.resources().insert(RGravity(-9.8));
///     }
///     
///     // you can call these methods whatever, but `on_{event}` is
///     // good practice
///     fn on_update(app: &mut Application)
///     {
///         if let Some(stage) = app.stage::<GameStage>()
///         {
///             let dt = app.time().dt();
///             let g  = app.res().get::<RGravity>();
///
///             let q = <Write<CVel>>::query();
///             
///             for mut vel in q.iter_mut(&mut stage.registry)
///             {
///                 vel += g * dt;
///             }
///             
///             let q = <(Write<CPos>, Read<CVel>)>::query();
///             
///             for (mut pos, vel) in q.iter_mut(&mut stage.registry)
///             {
///                 pos += vel * dt;
///             }
///         }
///     }
///     
///     // event handlers also don't need to be inside an `impl` block,
///     // but it's good for scoping
///     fn on_quit(app: &mut Application)
///     {
///         app.resources().remove::<RGravity>();
///     }
/// }
/// ```
pub trait System: 'static
{
    fn register(handlers: &mut Systems);
}

/// maps events to a list of sorted handlers
pub struct Systems
{
    /// < event id, sorted vec of (order, function) >
    map: HashMap<TypeId, Vec<(isize, EventHandler)>>,
}

/// function signature for an event handler. to be
/// used within the `System` trait.
pub type EventHandler = fn(&mut Application);

impl Systems
{
    /// create a new event-handlers systems map
    pub(crate) fn create() -> Self
    {
        Self { map: Default::default() }
    }

    /// insert an event handler in the events system
    pub fn insert<T: 'static>(&mut self, order: isize, func: EventHandler)
    {
        // event id
        let id = TypeId::of::<T>();

        // case: first handler for event
        if !self.map.contains_key(&id)
        {
            self.map.insert(id, vec![]);
        }
        let handlers = self.map.get_mut(&id).unwrap();

        // insert and sort
        handlers.push((order, func));
        handlers.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    }

    /// recursively process events
    pub(crate) fn process(&self, app: &mut Application)
    {
        // retrieve and replace events
        let queue = &mut app.events().queue;
        let events = std::mem::replace(queue, Default::default());

        // break recursion
        if events.is_empty()
        {
            return;
        }

        // process all events
        while let Ok(evt_id) = events.pop()
        {
            // get handlers list
            if let Some(handlers) = self.map.get(&evt_id)
            {
                // go through all handlers
                for (_, func) in handlers
                {
                    func(app);
                }
            }
        }
        
        // recurse
        self.process(app);
    }
}