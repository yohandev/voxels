use super::*;

use std::collections::HashMap;

use event_loop::*;

/// The application is the core of the game. Most of its functions,
/// apart from a few internal things, are pure ECS. The most
/// relevant function in this struct is Application::run, and
/// everything explains itself after that.
pub struct Application
{
    universe: Universe,
    resources: Resources,

    events: EventMap,

    worlds: WorldList
}

pub(crate) type EventMap    = HashMap<&'static str, Vec<Schedule>>;
pub(crate) type WorldList   = Vec<World>;

impl Application
{
    /// create and run an application
    /// this calls the Game::build() function in your
    /// game state, which should initialize resources
    /// and add all systems.
    pub fn run<T>() where T : Game + 'static
    {
        // create app
        let mut app = Self::create();

        // add default resources
        app
            .resources()
            .insert(resources::EventsQueue::new());

        // build game
        T::build(&mut app);

        // run game
        EventLoop::new().run
        (
            move |event, window_target, flow|
            {
                // poll events until quit
                *flow = ControlFlow::Poll;
                
                // special system: create windows
                systems::system_create_window(&mut app, window_target);

                // push current event into loop
                if let Some(static_event) = event.to_static()
                {
                    app
                        .resources()
                        .insert(resources::WinitEvent(static_event));
                }

                // invoke systems for new event
                app.invoke(events::APP_POLL_EVENT);

                // process events
                systems::system_process_invokes(&mut app, flow);
            }
        );
    }

    /// creates a world and registers it to this application.
    /// it's also possible to create a world directly from the
    /// universe, but that wouldn't make it a candidate for
    /// application events.
    pub fn create_world(&mut self) -> &mut World
    {
        let world = self.universe_mut().create_world();

        self.worlds.push(world);
        self.worlds.last_mut().unwrap()
    }

    /// get the universe(ECS world factory) for this app
    pub fn universe_mut(&mut self) -> &mut Universe
    {
        &mut self.universe
    }

    /// get the universe(ECS world factory) for this app
    pub fn universe(&self) -> &Universe
    {
        &self.universe
    }

    /// get all the active and registered worlds for this app
    pub fn worlds(&self) -> &WorldList
    {
        &self.worlds
    }

    /// get all the active and registered worlds for this app
    pub fn worlds_mut(&mut self) -> &mut WorldList
    {
        &mut self.worlds
    }

    /// get the resources for this app
    pub fn resources(&mut self) -> &mut Resources
    {
        &mut self.resources
    }

    /// register a schedule(collection of systems that run at once) for an event.
    /// schedules registered for the same event will be invoked in order of
    // registration: first come first serve.
    pub fn register_schedule(&mut self, event: &'static str, schedule: Schedule)
    { 
        // register event
        if !self.events.contains_key(event)
        {
            self.events.insert(event, Default::default());
        }

        // register system schedule
        self.events
            .get_mut(event)
            .unwrap()
            .push(schedule);
    }

    /// register a single system for an event. this creates a schedule behind the scenes,
    /// which runs synchronously from other schedules. to take advantage of legion's
    /// parallel systems executor, make your own schedule and use
    /// Application::register_schedule()
    pub fn register_system(&mut self, event: &'static str, system: Box<dyn Schedulable>)
    {
        let schedule = Schedule::builder()
            .add_system(system)
            .build();
        
        self.register_schedule(event, schedule);
    }

    /// invoke an event.
    /// does nothing if the event doesn't exist or has no bound systems.
    pub fn invoke(&mut self, event: &'static str)
    {
        if let Some(schedules) = self.events.get_mut(event)
        {
            for world in &mut self.worlds
            {
                for schedule in schedules.iter_mut()
                {
                    schedule.execute(world, &mut self.resources);
                }   
            }
        }
    }

    /// add common systems. they won't interfere with any of your components
    /// and may impact some ezgame provided ones, but adding these might prevent
    /// some headaches and weird behaviours.
    /// # list of systems
    /// - window_system: processes events for the Window component
    pub fn add_default_systems(&mut self)
    {
        self.register_system(events::APP_POLL_EVENT, systems::window_system());
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