use super::*;

pub struct Application
{
    app_loop: Option<EventLoop<()>>,

    windows: Vec<Window>,
    input: Input,
}

pub trait State
{
    fn on_update(&mut self, input: &Input);
    fn on_render(&self, window: &mut Window);
}

impl Application
{
    pub fn create(name: &str) -> ApplicationBuilder
    {
        ApplicationBuilder::new(name)
    }

    pub fn new() -> Self
    {
        Self
        {
            app_loop: Some(EventLoop::new()),
            windows: Vec::new(),
            input: Input::new(),
        }
    }

    pub fn create_window(&mut self, name: &str, size: uint2) -> &Window
    {
        use winit::window::WindowBuilder;
        use winit::dpi::LogicalSize;

        self.windows.push
        (
            Window
            {
                winit:
                {
                    WindowBuilder::new()
                        .with_title(name)
                        .with_inner_size(LogicalSize::new(size.x, size.y))
                        .build(self.app_loop.as_ref().unwrap())
                        .unwrap()
                },
                size,
                focused: false,
            }
        );

        self.windows
            .last()
            .unwrap()
    }

    pub fn run<T: State + 'static>(mut self, mut state: T)
    {
        let app_loop = self.app_loop
            .take()
            .unwrap();

        app_loop.run
        (
            move |event, _, flow|
            {
                *flow = ControlFlow::Poll;

                // pass event to window(s)
                for window in &mut self.windows
                {
                    window.process_events(&event, flow, &state);
                }

                // pass event to input
                self.input.process_events(&event, flow, &mut state);
            }
        )
    }
}