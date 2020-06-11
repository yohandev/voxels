use super::*;

pub struct ApplicationBuilder<'a>
{
    name: &'a str,
    
    app: Application,
}

impl<'a> ApplicationBuilder<'a>
{
    pub fn new(name: &'a str) -> Self
    {
        Self
        {
            name,
            app: Application::new()
        }
    }

    pub fn with_graphics(mut self, w: u32, h: u32) -> Self
    {
        self.app.create_window(self.name, uint2::new(w, h));
        self
    }

    pub fn run<T: State + 'static>(self, mut state: T)
    {
        self.app.run(state)
    }
}