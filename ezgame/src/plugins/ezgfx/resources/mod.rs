/// renderer resource, which in this case is just
/// an alias to the ezgfx::Renderer but wrapped by
/// an option in case the resource doesn't exist. it's
/// added by default whenever a new window is created.
pub type Renderer = Option<ezgfx::Renderer>;