use crate::*;

/// The application is the core of the game. Most of its functions,
/// apart from a few internal things, are pure ECS. The most
/// relevant function in this struct is Application::run, and
/// everything explains itself after that.
pub struct Application
{
    universe: legion::Universe,
    resources: legion::Resources,

    systems: Systems,

    worlds: WorldList
}

/// alias for list of worlds
pub(crate) type WorldList = Vec<legion::World>;

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

        // build game
        T::build(&mut app);

        // start event
        //app.invoke(events::APP_START);

        use winit::event_loop::*;

        // run game
        EventLoop::new().run
        (
            move |event, window_target, flow|
            {
                // poll events until quit
                *flow = ControlFlow::Poll;
                
                // special system: create windows
                plugins::winit::systems::system_create_window(&mut app, window_target);

                // push current event into loop
                if let Some(static_event) = event.to_static()
                {
                    app
                        .resources()
                        .insert(resources::WinitEvent(static_event));
                }

                // invoke systems for new event
                //app.invoke(events::APP_POLL);

                // process events
                systems::system_process_invokes(&mut app, flow);
            }
        );
    }

    /// creates a world and registers it to this application.
    /// it's also possible to create a world directly from the
    /// universe, but that wouldn't make it a candidate for
    /// application events.
    pub fn create_world(&mut self) -> &mut legion::World
    {
        let world = self.universe.create_world();

        self.worlds.push(world);
        self.worlds.last_mut().unwrap()
    }

    /// add a system to this application. systems are ran in the
    /// order they're added.
    pub fn add_system<T: System>(&mut self, sys: T)
    {
        self.systems.insert(self, sys);
    }

    /// add a system bundle to this application.
    pub fn add_bundle<T: SystemBundle>(&mut self, bundle: T)
    {
        bundle.build(self);
    }

    /// get all the active and registered worlds for this app
    pub fn worlds(&mut self) -> &mut WorldList
    {
        &mut self.worlds
    }

    /// get the resources for this app
    pub fn resources(&mut self) -> &mut legion::Resources
    {
        &mut self.resources
    }

    /// add common systems and resources. they won't interfere with any of your
    /// components and may impact some ezgame provided ones, but adding these
    /// will prevent some headaches and weird behaviours.
    /// # list of systems
    /// - window_system: processes events for the Window component, from winit plugin.
    /// - input_system: processes events and caches input states
    /// - time_system: processes events and calls game loop events
    /// # list of resources
    /// - EventsQueue: used by the engine to queue and poll system events
    /// - Input: interprets and caches key and button presses
    /// - Time: caches delta time and other game time information
    /// - None Window
    pub fn add_defaults(&mut self)
    {
        // resources
        // self.resources().insert(resources::EventsQueue::new());
        // self.resources().insert(resources::Input::new());
        // self.resources().insert(resources::Time::new());
        // self.resources().insert(plugins::winit::resources::Window::None);

        // // systems
        // self.register_schedule
        // (
        //     events::APP_POLL,
        //     legion::Schedule::builder()
        //         .add_system(plugins::winit::systems::window_system())
        //         .add_system(systems::input_system())
        //         .add_system(systems::time_system())
        //         .build()
        // );
    }

    /// create a new app
    fn create() -> Self
    {
        Self
        {
            universe: legion::Universe::new(),
            resources: legion::Resources::default(),
            systems: Systems::new(),
            worlds: Vec::default()
        }
    }
}