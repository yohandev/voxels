/// window component, representing an instance of a window.
/// represents a window creation request when first added,
/// then stores either a pointer to the OS window or an
/// error. 
#[derive(Debug)]
pub struct Window(Option<WindowResult>);

/// represents the size of a window. this component is only ever
/// useful when coupled with the Window component. When added
/// alongside the Window component, this size component is used
/// as the initial size of the window. After that, it reports
/// the associated window's current size.
#[derive(Debug, Copy, Clone)]
pub struct WindowSize
{
    pub width: u32,
    pub height: u32
}

/// represents the title of a window. when a Window component is
/// added to the same entity as this, the window is initialized
/// with this component's title.
#[derive(Debug, Clone)]
pub struct WindowTitle
{
    pub value: String
}

/// alias to winit's WindowBuilder return value
type WindowResult = Result<winit::window::Window, winit::error::OsError>;

impl Window
{
    /// request a new window. add this component to an entity to
    /// request a window, which is added to this component a frame
    /// or so later. the created window can be retreived through
    /// this component's get() method.
    pub fn request() -> Self
    {
        Self(None)
    }

    /// get the winit window behind this component. if the window
    /// hasn't been initiated yet, or ended up erroring, None is
    /// returned.
    pub fn get(&self) -> Option<&winit::window::Window>
    {
        if let Some(res) = &self.0
        {
            if let Ok(win) = &res
            {
                Some(win)
            }
            else
            {
                None
            }
        }
        else
        {
            None
        }
    }

    /// has this window been initialized, either into a window or
    /// an error. 
    pub fn is_init(&self) -> bool
    {
        self.0.is_some()
    }

    /// get the winit error behind this component. if the window
    /// hasn't been initiated yet, or ended up succesful, None is
    /// returned.
    pub fn err(&self) -> Option<&winit::error::OsError>
    {
        if let Some(res) = &self.0
        {
            if let Err(err) = &res
            {
                Some(err)
            }
            else
            {
                None
            }
        }
        else
        {
            None
        }   
    }

    pub(crate) fn init(&mut self, res: WindowResult)
    {
        self.0 = Some(res);
    }
}

impl Default for WindowSize
{
    fn default() -> Self
    {
        Self { width: 800, height: 600 }
    }
}

impl Default for WindowTitle
{
    fn default() -> Self
    {
        Self { value: "ezgame window".to_string() }
    }
}