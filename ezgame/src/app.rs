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
        app.invoke(evt::START);

        // run game
        winit::event_loop::EventLoop::new().run
        (
            move |event, window_target, flow|
            {
                // poll events until quit
                *flow = winit::event_loop::ControlFlow::Poll;
                
                // poll window requests
                window::create_window(&mut app, window_target);

                // push current event into loop
                if let Some(static_event) = event.to_static()
                {
                    // insert latest event
                    app.resources().insert(static_event);

                    // invoke systems for new event
                    app.invoke(evt::POLL);
                }

                // process events
                app.systems.process(&mut app.active, &mut app.resources);
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

    /// get the active entity registry
    pub fn registry(&mut self) -> &mut ecs::Registry
    {
        &mut self.active
    }

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
        resources.insert(ecs::REvents::default());

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
    pub(crate) fn invoke(&mut self, e: ecs::Event)
    {
        self
            .resources()
            .get::<ecs::REvents>()
            .unwrap()
            .push(e);
    }
}