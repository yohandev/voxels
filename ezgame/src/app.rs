use super::*;

use std::collections::HashMap;

use event::Event as WinitEvent;
use event_loop::*;

pub struct Application
{
    universe: Universe,
    resources: Resources,

    events: EventMap,

    worlds: WorldList
}

pub(crate) type EventMap    = HashMap<&'static str, Schedule>;
pub(crate) type WorldList   = Vec<World>;

impl Application
{
    /// create and run an application
    pub fn run<T>() where T : Game + 'static
    {
        // create app
        let mut app = Self::create();

        // build game
        T::build(&mut app);

        // run game
        EventLoop::new().run
        (
            move |event, window_target, flow|
            {
                // poll events until quit
                *flow = ControlFlow::Poll;
                
                // special system: quit application
                // TODO

                // special system: create windows
                systems::system_create_window(&mut app.worlds, window_target);

                // push current event into loop
                if let Some(static_event) = event.to_static()
                {
                    app
                        .resources()
                        .insert(static_event);
                }

                // invoke systems for new event
                app.invoke(events::APP_POLL_EVENT);
            }
        );
    }

    /// creates a world and registers it to this application.
    /// it's also possible to create a world directly from the
    /// universe, but that wouldn't make it a candidate for
    /// application events.
    pub fn create_world(&mut self) -> &mut World
    {
        let world = self.universe().create_world();

        self.worlds.push(world);
        self.worlds.last_mut().unwrap()
    }

    /// get the universe(ECS world factory) for this app
    pub fn universe(&mut self) -> &mut Universe
    {
        &mut self.universe
    }

    /// get the resources for this app
    pub fn resources(&mut self) -> &mut Resources
    {
        &mut self.resources
    }

    /// register a schedule(collection of systems that run at once) for an event.
    /// panics if the event has already been registered. 
    pub fn register(&mut self, event: &'static str, systems: Schedule)
    {
        // assert event doesn't exist
        if self.events.contains_key(event)
        {
            panic!("cannot register schedule for event {} because it already exists!", event);
        }

        // register system
        self.events.insert(event, systems);
    }

    /// invoke an event.
    /// does nothing if the event doesn't exist or has no bound systems.
    pub fn invoke(&mut self, event: &'static str)
    {
        if let Some(schedule) = self.events.get_mut(event)
        {
            for world in &mut self.worlds
            {
                schedule.execute(world, &mut self.resources);
            }
        }
    }

    /// create a new app
    fn create() -> Self
    {
        Self
        {
            universe: Universe::new(),
            resources: Resources::default(),

            events: EventMap::new(),

            worlds: Vec::default()
        }
    }
}