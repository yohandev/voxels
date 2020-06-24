use super::ecs::Event;

/// event invoked by the application whenever a raw event
/// is polled. this is called at least once per frame, and
/// should not cause expensive computations. the polled
/// event can be retreived in the application's resources
/// by giving the system a ReadResource<winit::event::Event>.
pub const POLL: Event = Event::new("ezgame_poll");

/// event invoked by the application right after the Game::begin()
/// call. It's only run once and can be used to initialize an ECS
/// scene or resources outside the game module.
pub const START: Event = Event::new("ezgame_start");

/// event received by subscribed user systems, and finally the
/// application. as the name implies, this event quits
/// the game and closes all windows, etc.
pub const QUIT: Event = Event::new("ezgame_quit");

/// event invoked by ezgame's time system. this is called
/// as often as possible, and can be vsync-capped by ezgfx.
/// such would yield ~60fps on most devices. Either way,
/// you can't rely on this event's timings, and it's important
/// to use the ezgame::resources::Time resource to retreive
/// the delta time. 
pub const UPDATE: Event = Event::new("ezgame_update");

/// event invoked by ezgame's window system. it's called
/// with the os' vsync rate, and is useful for rendering
/// pre/post processing actions.
pub const RENDER: Event = Event::new("ezgame_render");