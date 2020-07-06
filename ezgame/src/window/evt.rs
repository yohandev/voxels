/// event invoked when an RWindow resource
/// has been succesfully initialized. This
/// is useful for window-dependent systems,
/// such as the ezgfx SRendererInit.
pub const CREATED: &str = "ezgame_window_created";

/// event invoked when any window is resized.
pub struct ResizedEvent(pub u32, pub u32);