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

type EventMap    = HashMap<&'static str, Schedule>;
type WorldList   = Vec<World>;

impl Application
{
    /// create and run an application
    pub fn run<T>() where T : Game + 'static
    {
        // create app
        let mut app = Self::create();

        // build game
        T::build(&mut app);

        let mut window = None;

        // run game
        EventLoop::new().run
        (
            move |event, window_target, flow|
            {
                *flow = ControlFlow::Poll;
                
                if window.is_none()
                {
                    window = Some
                    (
                        window::WindowBuilder::new()
                            .with_inner_size(dpi::PhysicalSize::new(800, 600))
                            .with_resizable(true)
                            .with_title("test")
                            .with_visible(true)
                            .build(window_target)
                    );
                }
                let evt_str = format!("{:?}", &event);

                if event.to_static().is_none()
                {
                    println!("event {:?} could not be static-ized :(", evt_str);
                }
            }
        );
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

/// events and control flow of frame in the game loop
pub struct LoopSnapshot(&'static WinitEvent<'static, ()>);