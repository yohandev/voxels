pub mod evt;
mod res;
mod sys;

pub use res::*;
pub use sys::*;

pub use ezgfx::*;

impl super::Application
{
    /// short-cut for
    /// ```rust
    /// app.res().get::<RGraphics>().unwrap()
    /// ```
    /// get the app's graphics resource, which might not
    /// be there at all.
    pub fn gfx(&self) -> super::ecs::ResFetch<'_, RGraphics>
    {
        self.res().get::<RGraphics>().unwrap()
    }

    /// short-cut for
    /// ```rust
    /// app.res_mut().get_mut::<RGraphics>().unwrap()
    /// ```
    /// get the app's graphics resource, which might not
    /// be there at all.
    pub fn gfx_mut(&mut self) -> super::ecs::ResFetchMut<'_, RGraphics>
    {
        self.res_mut().get_mut::<RGraphics>().unwrap()
    }
}