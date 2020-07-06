/// Window resource, which is simply an alias
/// for winit's window, but wrapped in an option
/// in case the window doesn't exist.
pub type RWindow = winit::window::Window;

/// resource used to request a window. It will
/// place an RWindow resource into the world,
/// silently overriding it the already existing
/// one if any.
///
/// this resource is removed from the world once
/// window creation is complete, and panics if
/// creation was unsuccesful.
pub struct RWindowRequest
{
    /// initial width of the window
    pub width: u32,

    /// initial height of the window
    pub height: u32,

    /// display title of the window
    pub title: &'static str
}

impl RWindowRequest
{
    /// begin a new window request
    pub fn new() -> Self
    {
        Self::default()
    }

    /// set the initial width of the window
    pub fn width(mut self, n: u32) -> Self
    {
        self.width = n;
        self
    }

    /// set the initial height of the window
    pub fn height(mut self, n: u32) -> Self
    {
        self.height = n;
        self
    }

    /// set the display title of the window
    pub fn title(mut self, n: &'static str) -> Self
    {
        self.title = n;
        self
    }
}

impl Default for RWindowRequest
{
    fn default() -> Self
    {
        Self
        {
            width: 600,
            height: 400,
            title: "",
        }
    }
}