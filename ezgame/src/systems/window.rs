use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowBuilder;
use winit::dpi::PhysicalSize;

use crate::components::{ Window, WindowSize, WindowTitle };
use crate::resources::{ WinitEvent, EventsQueue };
use crate::Application;

use super::*;

/// special system that's called by the Application itself to
/// create windows for entities with !init() Window components.
pub(crate) fn system_create_window(app: &mut Application, target: &EventLoopWindowTarget<()>)
{
    // query window components
    let query = <(Write<Window>, TryRead<WindowSize>, TryRead<WindowTitle>)>::query();

    // whether event should be invoked
    let mut success = false;

    // iterate every world non-exlusively
    for world in app.worlds_mut()
    {
        for (mut win, size, title) in query.iter_mut(world)
        {
            // ignore already initialized
            if win.is_init()
            {
                continue;
            }

            // window builder
            let mut build = WindowBuilder::new();

            // window builder: size
            if let Some(size) = size
            {
                build = build.with_inner_size(PhysicalSize::new(size.width, size.height));
            }

            // window builder: title
            if let Some(title) = title
            {
                build = build.with_title(title.value.to_owned());
            }

            // finally, build window into component
            win.init(build.build(target));

            // keep track of whether event should be emmited
            success |= win.get().is_some();
        }
    }
    if success
    {
        // invoke event
        app.invoke(crate::events::APP_WINDOW_CREATION_EVENT);
    }
}

/// system that processes windows' events.
pub fn window_system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("window_system")
        .read_resource::<WinitEvent>()
        .write_resource::<EventsQueue>()
        .with_query(<(Read<Window>, TryWrite<WindowSize>)>::query())
        .build(|_, world, (event, invoke), query|
        {
            for (window, mut size) in query.iter_mut(world)
            {
                if let Some(window) = window.get()
                {
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
                                winit::event::WindowEvent::Resized(resize) =>
                                {
                                    if let Some(size) = &mut size
                                    {
                                        size.width = resize.width;
                                        size.height = resize.height;
                                    }
                                }
                                winit::event::WindowEvent::CloseRequested =>
                                {
                                    invoke.invoke(crate::events::APP_QUIT);
                                }
                                winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } =>
                                {
                                    if let Some(size) = &mut size
                                    {
                                        size.width = new_inner_size.width;
                                        size.height = new_inner_size.height;
                                    }
                                },
                                _ => {}
                            }
                        }
                        winit::event::Event::RedrawRequested(_) =>
                        {
                            invoke.invoke(crate::events::APP_RENDER_EVENT);
                        }
                        winit::event::Event::MainEventsCleared =>
                        {
                            window.request_redraw();
                        }
                        _ => {}
                    };
                }
            }
        })
}