use crate::framework::*;
use crate::ezmath::*;

pub struct Game
{
    app: Application
}

impl Game
{
    pub const NAME: &'static str = "Voxels Game";

    pub fn new() -> Self
    {
        Self { app: Application::create() }
    }

    pub fn with_graphics(mut self, w: u32, h: u32) -> Self
    {
        self.app.create_window(Self::NAME, uint2::new(w, h));
        self
    }

    pub fn run(self)
    {
        self.app.run()
    }
}