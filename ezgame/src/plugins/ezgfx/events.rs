/// event invoked by the the ezgfx plugin whenever a
/// renderer is ready. this is useful for initializing
/// render pipelines, textures, and other assets. it's
/// invoked as many times as there are ezgfx::*::Renderer
/// components.
pub const EZGFX_READY: &str = "ezgfx_ready_event";