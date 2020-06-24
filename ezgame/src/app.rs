use super::*;

/// The application is the core of the game. Most of its functions,
/// apart from a few internal things, are pure ECS. The most
/// relevant function in this struct is Application::run, and
/// everything explains itself after that.
pub struct Application
{
    resources:  ecs::Resources,
    systems:    ecs::EventSystems,
    active:     ecs::Registry,
}

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
        app.build::<T>();

        // prepare systems
        app.prepare();

        // start event
        app.invoke(&evt::START);

        // run game
        winit::event_loop::EventLoop::new().run
        (
            move |event, window_target, flow|
            {
                // poll events until quit
                *flow = winit::event_loop::ControlFlow::Poll;
                
                // special system: create windows
                //plugins::winit::systems::system_create_window(&mut app, window_target);

                // push current event into loop
                if let Some(static_event) = event.to_static()
                {
                    app
                        .resources()
                        .insert(static_event);
                }

                // invoke systems for new event
                app.invoke(&evt::POLL);

                // process events
                app.systems.process(&mut app.active, &mut app.resources);
                //systems::system_process_invokes(&mut app, flow);
            }
        );
    }

    /// get the resources for this app.
    pub fn resources(&mut self) -> &mut ecs::Resources
    {
        &mut self.resources
    }

    /// get the systems manager for this app
    pub fn systems(&mut self) -> &mut ecs::EventSystems
    {
        &mut self.systems
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
    // pub fn add_defaults(&mut self)
    // {
    //     // resources
    //     self.resources().insert(resources::EventsQueue::new());
    //     self.resources().insert(resources::Input::new());
    //     self.resources().insert(resources::Time::new());
    //     self.resources().insert(plugins::winit::resources::Window::None);

    //     // systems
    //     self.register_schedule
    //     (
    //         events::APP_POLL,
    //         Schedule::builder()
    //             .add_system(plugins::winit::systems::window_system())
    //             .add_system(systems::input_system())
    //             .add_system(systems::time_system())
    //             .build()
    //     );
    // }

    /// create a new app
    fn create() -> Self
    {
        // resources and systems
        let mut resources = ecs::Resources::default();
        let systems = ecs::EventSystems::default();
        
        // universe
        let factory = ecs::RegistryFactory::new();

        // create active registry
        let active = factory.create_world();

        // insert engine resources
        resources.insert(factory);

        // return
        Self { resources, systems, active }
    }

    /// build the game on this app
    fn build<T: Game>(&mut self)
    {
        T::build(self);
    }

    /// prepare currently added systems
    fn prepare(&mut self)
    {
        self.systems.build(&mut self.resources);
    }

    /// shortcut for `app.resources.get<REvents>.push`
    fn invoke(&mut self, e: &ecs::Event)
    {
        self
            .resources()
            .get::<ecs::REvents>()
            .unwrap()
            .push(e);
    }
}