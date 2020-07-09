pub mod evt;
mod res;
mod sys;

pub use res::*;
pub use sys::*;

impl super::Application
{
    /// short-cut for
    /// ```rust
    /// app.res().get::<RTime>().unwrap()
    /// ```
    /// get the app's time resource, which should normally
    /// always be there and valid.
    pub fn time(&self) -> super::ecs::ResFetch<'_, RTime>
    {
        self.res().get::<RTime>().unwrap()
    }
}