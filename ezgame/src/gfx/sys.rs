use crate::window::*;
use crate::ecs::*;
use crate::*;
use super::*;

/// system that initializes the RGraphics
/// resource whenever a window is created
/// and resizes the framebuffer when the
/// window is resized
pub struct SGraphics;

impl System for SGraphics
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<crate::evt::Start>(-8999, Self::on_start);
        handlers.insert::<window::evt::Resized>(-9999, Self::on_window_resize);
        handlers.insert::<window::evt::Created>(-9999, Self::on_window_created);
    } 
}

impl SGraphics
{
    fn on_start(app: &mut Application)
    {
        app.resources().insert(super::RGraphics::None);
    }

    fn on_window_created(app: &mut Application)
    {
        // retrieve resources
        let (r_win, mut r_gfx) = app.fetch_mut::<(Read<RWindow>, Write<RGraphics>)>();

        // retrieve window
        if let Some(win)  = &*r_win
        {
            // window size
            let size = win.inner_size();

            // create renderer
            *r_gfx = Some(ezgfx::Renderer::from_window(win, size.width, size.height));
            
            // invoke event
            app.events().push::<super::evt::Ready>();
        }
    }

    fn on_window_resize(app: &mut Application)
    {
        // retrieve resources
        let (r_win, mut r_gfx) = app.fetch_mut::<(Read<RWindow>, Write<RGraphics>)>();

        // renderer is initialized
        if let Some(gfx) = &mut *r_gfx
        {
            // get window size
            let size = r_win
                .as_ref()
                .unwrap()
                .inner_size();

            // update swapchain size
            gfx.resize(size.width, size.height);
        }
    }
}