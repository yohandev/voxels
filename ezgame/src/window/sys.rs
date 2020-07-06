use crate::ecs::*;
use crate::evt::*;
use super::*;

/// system that processes raw winit events
/// regarding windows, then invokes events.
pub struct SWindow;

impl System<PollEvent> for SWindow
{
    const ORDER: isize = 9999;

    fn run(app: &mut crate::Application)
    {
        // ignore is no window is present
        if !app.res().contains::<RWindow>()
        {
            return;
        }
        let r_win = app
            .res()
            .get::<RWindow>()
            .unwrap();
        let r_poll = app
            .res()
            .get::<crate::RWinitPoll>()
            .unwrap();

        // process winit events
        match &*r_poll
        {
            winit::event::Event::WindowEvent { window_id, event } =>
            {
                if *window_id != r_win.id()
                {
                    return;   
                }
                match event
                {
                    winit::event::WindowEvent::Resized(_) =>
                    {
                        app.invoke(evt::ResizedEvent);
                    }
                    winit::event::WindowEvent::CloseRequested =>
                    {
                        app.invoke(QuitEvent);
                    }
                    winit::event::WindowEvent::ScaleFactorChanged {..} =>
                    {
                        app.invoke(evt::ResizedEvent);
                    },
                    _ => {}
                }
            }
            winit::event::Event::MainEventsCleared =>
            {
                r_win.request_redraw();
            }
            _ => {}
        };
    }
}