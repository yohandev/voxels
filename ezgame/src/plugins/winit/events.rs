/// event invoked when an ezgame::plugins::winit::*::Window resource
/// has been succesfully initialized. This is useful for window-dependent
/// resources, such as the ezgfx plugin's Renderer.
pub const WINDOW_CREATION: &str = "winit_window_created";

/// event invoked when any window is resized.
pub const WINDOW_RESIZE: &str = "winit_window_resized";