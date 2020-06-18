use legion::prelude::*;

use crate::plugins::winit::resources::Window;

use crate::resources::EventsQueue;
use super::resources::Renderer;

/// system that initializes the Renderer resource
pub fn init_system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("ezgfx_renderer_init_system")
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

/// resizes the renderer's frame buffer
pub fn resize_system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("ezgfx_renderer_resize_system")
        .read_resource::<Window>()
        .write_resource::<Renderer>()
        .build(|_, _, (window, renderer), _|
        {
            // renderer not initialized
            if renderer.is_none()
            {
                return;
            }

            // get window size
            let size = window
                .as_ref()
                .unwrap()
                .inner_size();

            // update swapchain size
            renderer
                .as_mut()
                .unwrap()
                .resize(size.width, size.height);
        })
}