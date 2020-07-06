use crate::ecs::*;
use crate::evt::*;
use super::*;

/// system that processes raw winit events
/// regarding windows, then invokes events.
pub struct SWindow;

impl System<PollEvent> for SWindow
{
    const ORDER: isize = 9999;

    fn run(&mut self, app: &mut crate::Application, evt: &PollEvent)
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

        // process winit events
        match &evt.0
        {
            winit::event::Event::WindowEvent { window_id, event } =>
            {
                if *window_id != r_win.id()
                {
                    return;   
                }
                match event
                {
                    winit::event::WindowEvent::Resized(size) =>
                    {
                        app.invoke(evt::ResizedEvent(size.width, size.height));
                    }
                    winit::event::WindowEvent::CloseRequested =>
                    {
                        app.invoke(QuitEvent);
                    }
                    winit::event::WindowEvent::ScaleFactorChanged {new_inner_size, ..} =>
                    {
                        app.invoke(evt::ResizedEvent(new_inner_size.width, new_inner_size.height));
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