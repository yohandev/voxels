use legion::prelude::*;

use super::components::Renderer;
use crate::components::Window;

/// system that initializes the Renderer component
pub fn renderer_system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("ezgfx_renderer_system")
        .with_query(<(Write<Renderer>, Read<Window>)>::query())
        .build(|_, world, _, query|
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
                }
            }
        })
}