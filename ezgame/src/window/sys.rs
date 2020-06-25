use crate::ecs::*;
use crate::evt;

/// system that processes raw winit events
/// regarding windows, then invokes events.
pub struct SWindow;

impl System for SWindow
{
    const EVENT: Event = evt::POLL;
    const ORDER: Order = ord::HIGH;

    fn prepare(r: &mut Resources)
    {
        r.insert(super::RWindow::None);
    }

    fn exe() -> SysFn
    {
        // begin...
        sys("ezgame_window_system")
        // resources...
        .read_resource::<super::REvent>()
        .read_resource::<super::RWindow>()
        .read_resource::<REvents>()
        // system...
        .build(|_, _, (r_winit, r_window, r_events), _|
        {
            // ignore is no window is present
            if r_window.is_none()
            {
                return;
            }
            let win = r_window.as_ref().unwrap();

            // process winit events
            match &**r_winit
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
                            r_events.push(super::evt::RESIZED);
                        }
                        winit::event::WindowEvent::CloseRequested =>
                        {
                            r_events.push(evt::QUIT);
                        }
                        winit::event::WindowEvent::ScaleFactorChanged {..} =>
                        {
                            r_events.push(super::evt::RESIZED);
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
        })
    }
}