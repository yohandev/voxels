use crate::*;
use super::*;

/// system that initializes the ResRenderer resource
pub struct SysRenderer;

impl System for SysRenderer
{
    fn build(&self, app: &mut Application) -> SystemFn
    {
        app.resources().insert(ResRenderer::None);

        use crate::plugins::win::*;
        use crate::legion::*;

        SystemBuilder::new("ezgfx_renderer_system")
        // resources
        .read_resource::<ResWindow>()
        .read_resource::<ResWindowEvents>()
        .write_resource::<ResRenderer>()
        .build(|_, _, (window, window_events, renderer), _|
        {
            // no window = no renderer
            if window.is_none()
            {
                return;
            }

            // retrieve window
            let window = window.as_ref().unwrap();
            let size = window.inner_size();

            // create renderer
            if window_events.poll().any(|p| *p == WindowEvent::Created)
            {
                // create renderer
                **renderer = Some(ezgfx::Renderer::from_window(window, size.width, size.height));    
            }

            // resize renderer
            if window_events.poll().any(|p| *p == WindowEvent::Resized)
            {
                // update swapchain size
                renderer
                    .as_mut()
                    .unwrap()
                    .resize(size.width, size.height);
            }
        })
    }  
}