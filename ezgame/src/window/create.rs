use winit::event_loop::EventLoopWindowTarget;

use crate::*;

pub(crate) fn create_window(app: &mut Application, target: &EventLoopWindowTarget<()>)
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

        // place window
        app.resources().insert(Some(window));

        // if program hasn't panicked by then,
        // invoke event
        app.invoke(super::evt::CREATED);
    }
}