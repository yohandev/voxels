use winit::event_loop::{ EventLoop, ControlFlow };
use winit_input_helper::WinitInputHelper;
use winit::event::VirtualKeyCode;
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::Event;

use pixels::{ Pixels, SurfaceTexture };
use pixels::wgpu::Surface;

use std::time::Instant;

pub struct Window;

pub trait State
{
    fn render(&self, width: u32, height: u32, frame: &mut [u8]);
    fn update(&mut self, dt: f32, input: &WinitInputHelper);
}

impl Window
{
    pub fn create(name: &str, width: u32, height: u32, mut state: Box<dyn State>)
    {
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window =
        {
            let size = LogicalSize::new(width as f64, height as f64);

            WindowBuilder::new()
                .with_title(name)
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels =
        {
            let surf = Surface::create(&window);
            let tex = SurfaceTexture::new(width, height, surf);

            Pixels::new(width, height, tex).expect("could not create pixels instance")
        };

        let mut frame = 0;
        let mut time = Instant::now();

        event_loop.run
        (
            move |evt, _, ctrl_flow|
            {
                // render
                if let Event::RedrawRequested(_) = evt
                {
                    state.render(width, height, pixels.get_frame());

                    if pixels
                        .render()
                        .map_err(|e| panic!("pixels.render() failed: {}", e))
                        .is_err()
                    {
                        *ctrl_flow = ControlFlow::Exit;
                        return;
                    }
                    frame += 1;
                }

                // Handle input events
                if input.update(evt)
                {
                    // close
                    if input.key_pressed(VirtualKeyCode::Escape) || input.quit()
                    {
                        *ctrl_flow = ControlFlow::Exit;
                        return;
                    }

                    // resize
                    if let Some(size) = input.window_resized()
                    {
                        pixels.resize(size.width, size.height);
                    }

                    let now = Instant::now();
                    let dt = now.duration_since(time);
                    time = now;

                    // update handler
                    state.update(dt.as_secs_f32(), &input);

                    // request redraw
                    window.request_redraw();
                }
            }
        )
    }
}