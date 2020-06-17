use ezgame::plugins::ezgfx::*;
use ezgame::legion::*;

use crate::resources::gfx::*;

pub(super) fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("render_system")
        // components
        .with_query(<Write<components::Graphics>>::query())
        // resources
        .write_resource::<Option<SimpleGfxResources>>()
        // system
        .build(|_, world, res, query|
        {
            if res.is_none()
            {
                return;
            }

            for mut ctx in query.iter_mut(world)
            {
                ctx.render_pass
                (
                    |_, mut pass|
                    {
                        pass.begin_clear(0.1, 0.2, 0.3, 1.0);
                    }
                );
            }
        })
}