use super::*;

pub struct Window
{
    pub(super) winit: winit::window::Window,

    pub(super) size: uint2,
    pub(super) focused: bool,
}

impl Window
{
    pub fn _width(&self) -> u32 { self.size.x }
    pub fn _height(&self) -> u32 { self.size.y }   

    pub(super) fn process_events(&mut self, event: &Event<()>, flow: &mut ControlFlow)
    {
        match event
        {
            Event::WindowEvent { window_id, event } =>
            {
                if *window_id != self.winit.id()
                {
                    return;   
                }
                match event
                {
                    WindowEvent::Resized(size) =>
                    {
                        self.size.x = size.width;
                        self.size.y = size.height;
                    }
                    WindowEvent::Focused(focused) =>
                    {
                        self.focused = *focused;
                    }
                    WindowEvent::CloseRequested =>
                    {
                        *flow = ControlFlow::Exit;
                    }
                    WindowEvent::ScaleFactorChanged { scale_factor:_, new_inner_size } =>
                    {
                        self.size.x = new_inner_size.width;
                        self.size.y = new_inner_size.height;
                    },
                    _ => {}
                }
            }
            Event::RedrawRequested(_) =>
            {
                // -- render logic --
            }
            Event::MainEventsCleared =>
            {
                self.winit.request_redraw();
            }
            _ => {}
        }
    }
}