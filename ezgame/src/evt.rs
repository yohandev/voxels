/// event invoked by the application whenever a raw event
/// is polled. this is called at least once per frame, and
/// should not cause expensive computations. the polled
/// event can be retreived in the application's resources
/// by giving the system a ReadResource<winit::event::Event>.
pub struct PollEvent(pub winit::event::Event<'static, ()>);

/// event invoked by the application right after the Game::begin()
/// call. It's only run once and can be used to initialize an ECS
/// scene or resources outside the game module.
pub struct StartEvent;

/// event received by subscribed user systems, and finally the
/// application. as the name implies, this event quits
/// the game and closes all windows, etc.
pub struct QuitEvent;