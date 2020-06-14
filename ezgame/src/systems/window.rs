use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowBuilder;
use winit::dpi::PhysicalSize;

use crate::components::{ Window, WindowSize, WindowTitle };
use crate::app::WorldList;

use super::*;

/// system that processes windows' events.
pub struct WindowSystem;

/// special system that's called by the Application itself to
/// create windows for entities with !init() Window components.
pub(crate) fn system_create_window(worlds: &mut WorldList, target: &EventLoopWindowTarget<()>)
{
    // query window components
    let query = <(Write<Window>, TryRead<WindowSize>, TryRead<WindowTitle>)>::query();

    // iterate every world non-exlusively
    for world in worlds
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
        }
    }
}