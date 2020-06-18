/// event invoked by the application whenever a raw event
/// is polled. this is called at least once per frame, and
/// should not cause expensive computations. the polled
/// event can be retreived in the application's resources
/// by giving the system a ReadResource<winit::event::Event>.
pub const APP_POLL: &str = "app_poll_event";

/// event invoked by the application right after the Game::begin()
/// call. It's only run once and can be used to initialize an ECS
/// scene or resources outside the game module.
pub const APP_START: &str = "app_start_event";

/// event received by subscribed user systems, and finally the
/// application. as the name implies, this event quits
/// the game and closes all windows, etc.
pub const APP_QUIT: &str = "app_quit_event";

/// event invoked by ezgame's time system. this is called
/// as often as possible, and can be vsync-capped by ezgfx.
/// such would yield ~60fps on most devices. Either way,
/// you can't rely on this event's timings, and it's important
/// to use the ezgame::resources::Time resource to retreive
/// the delta time. 
pub const APP_UPDATE: &str = "app_update_event";

/// event invoked by ezgame's window system. it's called
/// with the os' vsync rate, and is useful for rendering
/// pre/post processing actions.
pub const APP_RENDER: &str = "app_render_event";