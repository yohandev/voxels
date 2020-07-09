use super::ecs::*;
use super::*;

/// The application is the core of the game. Most of its functions,
/// apart from a few internal things, are pure ECS. The most
/// relevant function in this struct is Application::run, and
/// everything explains itself after that.
pub struct Application
{
    factory:    RegistryFactory,
    states:     StateMachine,
    resources:  Resources,
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
                window::SWindow::create(&mut app, window_target);

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
    /// app.res().get::<REventQueue>().unwrap()
    /// ```
    /// get the app's event queue, which should normally
    /// always be there and valid.
    pub fn events(&self) -> ResFetch<'_, REventQueue>
    {
        self.res().get::<REventQueue>().unwrap()
    }

    /// get this app's state manager
    pub fn states(&mut self) -> &mut StateMachine
    {
        &mut self.states
    }

    /// switch to a state, or register it using its default.
    /// this will invoke the StateChange event if the state
    /// actually changed
    pub fn switch<T: State>(&mut self)
    {
        use std::any::*;

        if !self.states().is::<T>()
        {
            self.events().push::<evt::StateChanged>();

            let id = TypeId::of::<T>();

            // implicit register
            if !self.states.states.contains_key(&id)
            {
                let new = { Box::new(T::create(self)) };
                
                self.states.states.insert(id, new);
            }

            // switch
            self.states.active = Some(id);
        }
    }

    /// get an immutable list of the active
    /// state's registries(if any). this is
    /// for systems that operate on *all* states
    /// but need access to entity registries
    /// (which are stored in the state). if no
    /// state is active, an empty array is
    /// returned.
    pub fn registries(&self) -> &[&Registry]
    {
        if let Some(state) = &self.states.active
        {
            self.states.states
                .get(state)
                .unwrap()
                .registries()
        }
        else { &[] }
    }

    /// get a mutable list of the active
    /// state's registries(if any). this is
    /// for systems that operate on *all* states
    /// but need access to entity registries
    /// (which are stored in the state). if no
    /// state is active, an empty array is
    /// returned.
    pub fn registries_mut(&mut self) -> &[&mut Registry]
    {
        if let Some(state) = &self.states.active
        {
            self.states.states
                .get_mut(state)
                .unwrap()
                .registries_mut()
        }
        else { &[] }
    }

    /// create a new registry for this app
    pub fn create_registry(&mut self) -> Registry
    {
        self.factory.create_world()
    }

    /// create a new app
    fn create() -> Self
    {
        // universe
        let factory = RegistryFactory::new();

        // states
        let states = StateMachine::new();

        // resources
        let mut resources = Resources::default();

        // insert engine resources
        resources.insert(REventQueue::new());

        // return
        Self { factory, states, resources }
    }
}