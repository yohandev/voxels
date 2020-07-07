use super::ecs::*;
use super::*;

/// The application is the core of the game. Most of its functions,
/// apart from a few internal things, are pure ECS. The most
/// relevant function in this struct is Application::run, and
/// everything explains itself after that.
pub struct Application
{
    resources:  Resources,
    active:     Registry,
}

/// winit resource taken directly from the event loop
pub(crate) type RWinitPoll = winit::event::Event<'static, ()>;

impl Application
{
    /// create and run an application
    /// this calls the Game::build() function in your
    /// game state, which should initialize resources
    /// and add all systems.
    pub fn run<T>() where T : Game + 'static
    {
        // create app
        let mut app = Application::create();
        let mut sys = Systems::create();

        // build game
        T::build(&mut app, &mut sys);

        // start event
        app.events().push::<evt::Start>();

        // run game
        winit::event_loop::EventLoop::new().run
        (
            move |event, window_target, flow|
            {
                // poll events until quit
                *flow = winit::event_loop::ControlFlow::Poll;
                
                // poll window requests
                //window::create_window(&mut app, window_target);

                // push current event into loop
                if let Some(static_event) = event.to_static()
                {
                    // insert latest event
                    app.resources().insert(static_event);

                    // invoke systems for new event
                    app.events().push::<evt::Poll>();
                }

                // process events
                sys.process(&mut app);
            }
        );
    }

    /// get the resources for this app.
    /// this is the exact same as `Application::res_mut()`
    /// but prettier
    pub fn resources(&mut self) -> &mut Resources
    {
        &mut self.resources
    }

    /// get the resources for this app(immutable)
    pub fn res(&self) -> &Resources
    {
        &self.resources
    }

    /// get the resources for this app(mutable)
    pub fn res_mut(&mut self) -> &mut Resources
    {
        &mut self.resources
    }

    /// short-cut for:
    /// ```rust
    /// <(Read<RFoo>, Read<RBar>)>::fetch(app.res())
    /// ```
    /// used to fetch multiple resources at once without rust's
    /// borrowing constraints
    pub fn fetch<T: ResourceSet + legion::query::ReadOnly>(&self) -> T::PreparedResources
    {
        T::fetch(self.res())
    }

    /// short-cut for:
    /// ```rust
    /// <(Read<RFoo>, Write<RBar>)>::fetch_mut(app.res_mut())
    /// ```
    /// used to fetch multiple resources at once without rust's
    /// borrowing constraints
    pub fn fetch_mut<T: ResourceSet>(&mut self) -> T::PreparedResources
    {
        T::fetch_mut(self.res_mut())
    }

    /// short-cut for
    /// ```rust
    /// app.res_mut().get::<REventQueue>().unwrap()
    /// ```
    /// get the app's event queue, which should normally
    /// always be there and valid.
    pub fn events(&mut self) -> &mut REventQueue
    {
        todo!()
    }

    /// create a new app
    fn create() -> Self
    {
        // resources and systems
        let mut resources = ecs::Resources::default();
        
        // universe
        let factory = ecs::RegistryFactory::new();

        // create active registry
        let active = factory.create_world();

        // insert engine resources
        resources.insert(factory);
        //resources.insert(ecs::REvents::default());

        // return
        Self { resources, active }
    }

    /// shortcut for `app.resources.get<REvents>.push`
    pub fn invoke<T>(&self)
    {
        todo!();
        // self
        //     .resources()
        //     .get::<ecs::REvents>()
        //     .unwrap()
        //     .push(e);
    }
}