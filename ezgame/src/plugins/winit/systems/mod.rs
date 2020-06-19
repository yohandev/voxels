use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowBuilder;
use winit::dpi::PhysicalSize;

use crate::resources::{ WinitEvent, EventsQueue };
use super::resources::{ Window, WindowRequest };

use crate::Application;
use crate::legion::*;

/// special system that's called by the Application itself to
/// create windows for WindowRequest resources.
pub(crate) fn system_create_window(app: &mut Application, target: &EventLoopWindowTarget<()>)
{
    // get request, if any
    if let Some(request) = app.resources().remove::<WindowRequest>()
    {
        // build window
        let result = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(request.width, request.height))
            .with_title(request.title)
            .build(target);

        // unwrap success or error
        let window = result.unwrap();

        // place window
        app.resources().insert(Some(window));

        // if program hasn't panicked by then,
        // invoke event
        app.invoke(super::events::WINDOW_CREATION);
    }
}

/// system that processes windows' events.
pub fn window_system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("window_system")
        // resource
        .read_resource::<Window>()
        .read_resource::<WinitEvent>()
        .write_resource::<EventsQueue>()
        // system
        .build(|_, _, (window, event, invoke), _|
        {
            // ignore is no window is present
            if window.is_none()
            {
                return;
            }
            let window = window.as_ref().unwrap();

            // process winit events
            match &event.0
            {
                winit::event::Event::WindowEvent { window_id, event } =>
                {
                    if *window_id != window.id()
                    {
                        return;   
                    }
                    match event
                    {
                        winit::event::WindowEvent::Resized(_) =>
                        {
                            invoke.invoke(super::events::WINDOW_RESIZE);
                        }
                        winit::event::WindowEvent::CloseRequested =>
                        {
                            invoke.invoke(crate::events::APP_QUIT);
                        }
                        winit::event::WindowEvent::ScaleFactorChanged {..} =>
                        {
                            invoke.invoke(super::events::WINDOW_RESIZE);
                        },
                        _ => {}
                    }
                }
                winit::event::Event::MainEventsCleared =>
                {
                    window.request_redraw();
                }
                _ => {}
            };
        })
}