use crate::window::*;
use crate::ecs::*;

/// system that initializes the RGraphics
/// resource whenever a window is created.
pub struct SGraphicsInit;

/// system that resizes the framebuffer when
/// the window is resized.
pub struct SGraphicsResize;

impl System for SGraphicsInit
{
    const EVENT: Event = evt::CREATED;
    const ORDER: Order = ord::HIGH * 2;
    
    fn prepare(r: &mut Resources)
    {
        r.insert(super::RGraphics::None);
    }
    
    fn exe() -> SysFn
    {
        // begin...
        sys("ezgfx_init_system")
        // resources
        .read_resource::<RWindow>()
        .write_resource::<super::RGraphics>()
        .read_resource::<REvents>()
        // system
        .build(|_, _, (r_window, r_gfx, r_events), _|
        {
            // retrieve window
            let window = r_window.as_ref().unwrap();
            let size   = window.inner_size();

            // create renderer
            **r_gfx = Some(ezgfx::Renderer::from_window(window, size.width, size.height));
            
            // invoke event
            r_events.push(super::evt::READY);
        })
    }
}

impl System for SGraphicsResize
{
    const EVENT: Event = evt::RESIZED;
    const ORDER: Order = ord::HIGH * 2;

    fn prepare(r: &mut Resources)
    {
        r.insert(super::RGraphics::None);
    }

    fn exe() -> SysFn
    {
        // begin...
        sys("ezgfx_resize_system")
        // resources...
        .read_resource::<RWindow>()
        .write_resource::<super::RGraphics>()
        // system
        .build(|_, _, (r_window, r_gfx), _|
        {
            // renderer not initialized
            if r_gfx.is_none()
            {
                return;
            }

            // get window size
            let size = r_window
                .as_ref()
                .unwrap()
                .inner_size();

            // update swapchain size
            r_gfx
                .as_mut()
                .unwrap()
                .resize(size.width, size.height);
        })
    }
}