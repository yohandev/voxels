use crate::*;
use super::*;

/// system that processes windows' events
pub struct SysWindow;

impl System for SysWindow
{
    fn build(&self, app: &mut Application) -> SystemFn
    {
        app.resources().insert(ResWindow::None);
        app.resources().insert(ResWindowEvents::default());

        use self::legion::*;

        SystemBuilder::new("ezgame_window_system")
        // resource
        .read_resource::<ResWindow>()
        .read_resource::<ResWinitEvents>()
        .write_resource::<ResWindowEvents>()
        // system
        .build(|_, _, (window, winit_events, window_events), _|
        {
            // clear window events
            window_events.clear();

            // ignore is no window is present
            if window.is_none()
            {
                return;
            }
            let window = window.as_ref().unwrap();

            // process winit events
            for event in winit_events.poll()
            {
                if let Some(event) = process_event(event, window)
                {
                    // push window events
                    window_events.push(event);
                }
            }
        })
    }
}

/// process a winit event
fn process_event(event: &winit::event::Event<()>, window: &winit::window::Window) -> Option<WindowEvent>
{
    match &event
    {
        winit::event::Event::WindowEvent { window_id, event } =>
        {
            if *window_id != window.id()
            {
                return None;   
            }
            match event
            {
                winit::event::WindowEvent::Resized(_) => Some(WindowEvent::Resized),
                winit::event::WindowEvent::CloseRequested => Some(WindowEvent::Closed),
                winit::event::WindowEvent::ScaleFactorChanged {..} => Some(WindowEvent::Resized),
                winit::event::WindowEvent::Moved(_) => Some(WindowEvent::Moved),
                _ => None
            }
        }
        winit::event::Event::MainEventsCleared =>
        {
            window.request_redraw();
            
            None
        }
        _ => None
    }
}