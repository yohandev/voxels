/// Window resource, which is simply an alias for winit's
/// window, but wrapped in an option in case the window
/// doesn't exist.
pub type ResWindow = Option<winit::window::Window>;

/// resource used to request a window. It will place an
/// ezgame::resources::Window resource into the world,
/// silently overriding it the already existing one if any.
///
/// this resource is removed from the world once window
/// creation is complete, and panics if creation was
/// unsuccesful. 
pub struct ResWindowRequest
{
    /// initial width of the window
    pub width: u32,

    /// initial height of the window
    pub height: u32,

    /// display title of the window
    pub title: &'static str
}

/// window events that are cleared every frame then polled
/// by the window system.
#[derive(Default)]
pub struct ResWindowEvents(Vec<WindowEvent>);

/// window event stored in the window events resource
pub enum WindowEvent
{
    Resized(u32, u32),
    Moved(i32, i32),
    Closed,
}

/// raw winit events from the event loop, accumulated
/// whenever an event is polled, then cleared after all
/// systems are run.
#[derive(Default)]
pub struct ResWinitEvents(Vec<winit::event::Event<'static, ()>>);

impl ResWindowRequest
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

impl Default for ResWindowRequest
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

impl ResWindowEvents
{
    /// iterate over this frame's window events
    pub fn poll(&self) -> std::slice::Iter<'_, WindowEvent>
    {
        self.0.iter()
    }

    /// clear all events in this resource
    pub(crate) fn clear(&mut self)
    {
        self.0.clear();
    }

    /// push an event for this frame
    pub(crate) fn push(&mut self, e: WindowEvent)
    {
        self.0.push(e);
    }
}

impl ResWinitEvents
{
    /// iterate over this frame's winit events
    pub fn poll(&self) -> std::slice::Iter<'_, winit::event::Event<()>>
    {
        self.0.iter()
    }

    /// clear all events in this resource
    pub(crate) fn clear(&mut self)
    {
        self.0.clear();
    }

    /// push an event for this frame
    pub(crate) fn push(&mut self, e: winit::event::Event<'static, ()>)
    {
        self.0.push(e);
    }
}