pub mod components;
pub mod systems;
pub mod events;

pub use ezgfx::*;

impl crate::Application
{
    /// add ezgfx systems and resources. they won't interfere with any of your
    /// components and may impact some ezgame provided ones, but adding these
    /// gives you the capabilities of ezgfx binded to work with ECS.
    /// # list of systems
    /// - ezgfx_renderer_system: initializes the ezgfx::Renderer component
    pub fn add_plugin_ezgfx(&mut self)
    {
        self.register_system(crate::events::APP_WINDOW_CREATION, systems::renderer_system());
    }
}