mod res;
mod sys;

pub use res::*;
pub use sys::*;

/// window bundle that creates and manages the window
pub struct WindowBundle;

impl crate::SystemBundle for WindowBundle
{
    fn build(&self, app: &mut crate::Application)
    {
        app.add_system(SysWindow);
    } 
}