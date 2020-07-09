pub mod evt;

mod res;
mod sys;

pub use res::*;
pub use sys::*;

impl super::Application
{
    /// short-cut for
    /// ```rust
    /// app.res().get::<RWindow>().unwrap()
    /// ```
    /// get the app's window resource, which might not
    /// be there at all.
    pub fn window(&self) -> super::ecs::ResFetch<'_, RWindow>
    {
        self.res().get::<RWindow>().unwrap()
    }
}