use legion::prelude::*;

use crate::resources::EventsQueue;
use super::components::Graphics;
use crate::components::Window;

/// system that initializes the Renderer component
pub fn renderer_system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("ezgfx_renderer_system")
        .write_resource::<EventsQueue>()
        .with_query(<(Write<Graphics>, Read<Window>)>::query())
        .build(|_, world, invoke, query|
        {
            for (mut ctx, win) in query.iter_mut(world)
            {
                if ctx.is_init()
                {
                    continue;
                }

                if !win.is_init()
                {
                    continue;
                }

                if let Some(win) = win.get()
                {
                    ctx.init(win);
                    invoke.invoke(super::events::EZGFX_READY);
                }
            }
        })
}