use crate::ecs::Event;

/// event invoked by ezgame's time system. this is called
/// as often as possible, and can be vsync-capped by ezgfx.
/// such would yield ~60fps on most devices. Either way,
/// you can't rely on this event's timings, and it's important
/// to use the ezgame::resources::Time resource to retreive
/// the delta time. 
pub const UPDATE: Event = "ezgame_update";

/// event invoked by ezgame's window system. it's called
/// with the os' vsync rate, and is useful for rendering
/// pre/post processing actions.
pub const RENDER: Event = "ezgame_render";