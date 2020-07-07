use crate::ecs::*;
use crate::*;
use super::*;

/// system that processes raw winit events
/// regarding windows, then invokes events.
pub struct SWindow;

impl System for SWindow
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<crate::evt::Start>(-9999, Self::on_start);
        handlers.insert::<crate::evt::Poll>(-9999, Self::on_poll);
    }
}

impl SWindow
{
    fn on_start(app: &mut Application)
    {
        app.resources().insert(RWindow::None);
    }

    fn on_poll(app: &mut Application)
    {
        // fetch resources
        let (r_win, r_poll) = app.fetch::<(Read<RWindow>, Read<RWinitPoll>)>();

        // ignore is no window is present
        if r_win.is_none()
        {
            return;
        }
        let win = r_win.as_ref().unwrap();

        // process winit events
        match &*r_poll
        {
            winit::event::Event::WindowEvent { window_id, event } =>
            {
                if *window_id != win.id()
                {
                    return;   
                }
                match event
                {
                    winit::event::WindowEvent::Resized(_) =>
                    {
                        app.events().push::<super::evt::Resized>();
                    }
                    winit::event::WindowEvent::CloseRequested =>
                    {
                        app.events().push::<crate::evt::Quit>();
                    }
                    winit::event::WindowEvent::ScaleFactorChanged {..} =>
                    {
                        app.events().push::<super::evt::Resized>();
                    },
                    _ => {}
                }
            }
            winit::event::Event::MainEventsCleared =>
            {
                win.request_redraw();
            }
            _ => {}
        };
    }

    /// special engine-system that creates windows
    pub(crate) fn create(app: &mut Application, target: &winit::event_loop::EventLoopWindowTarget<()>)
    {
        use winit::window::WindowBuilder;
        use winit::dpi::PhysicalSize;

        // get request, if any
        if let Some(request) = app.resources().remove::<super::RWindowRequest>()
        {
            // build window
            let result = WindowBuilder::new()
                .with_inner_size(PhysicalSize::new(request.width, request.height))
                .with_title(request.title)
                .build(target);

            // unwrap success or error
            let window = result.unwrap();

            // explicit typing to make sure the resource is
            // correct
            let res: RWindow = Some(window);

            // place window
            app.resources().insert(res);

            // if program hasn't panicked by then,
            // invoke event
            app.events().push::<super::evt::Created>();
        }
    }
}