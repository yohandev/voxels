mod res;
mod sys;

pub use res::*;
pub use sys::*;

impl super::Application
{
    /// short-cut for
    /// ```rust
    /// app.res().get::<RInput>().unwrap()
    /// ```
    /// get the app's input resource, which should normally
    /// always be there and valid.
    pub fn input(&self) -> super::ecs::ResFetch<'_, RInput>
    {
        self.res().get::<RInput>().unwrap()
    }
}