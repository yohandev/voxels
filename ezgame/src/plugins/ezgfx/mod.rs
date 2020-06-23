mod sys;
mod res;

pub use sys::*;
pub use res::*;

pub use ezgfx::*;

/// render bundle that creates and manages the renderer.
/// depends on the window bundle.
pub struct RenderBundle;

impl crate::SystemBundle for RenderBundle
{
    fn build(&self, app: &mut crate::Application)
    {
        app.add_system(SysRenderer);
    } 
}