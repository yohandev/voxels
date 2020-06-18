use legion::prelude::*;

use crate::plugins::winit::resources::Window;

use crate::resources::EventsQueue;
use super::resources::Renderer;

/// system that initializes the Renderer resource
pub fn renderer_system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("ezgfx_renderer_system")
        // resource
        .read_resource::<Window>()
        .write_resource::<Renderer>()
        .write_resource::<EventsQueue>()
        // system
        .build(|_, _, (window, renderer, invoke), _|
        {
            // retrieve window
            let window = window.as_ref().unwrap();
            let size   = window.inner_size();

            // create renderer
            **renderer = Some(ezgfx::Renderer::from_window(window, size.width, size.height));
            
            // invoke event
            invoke.invoke(super::events::EZGFX_READY);
        })
}