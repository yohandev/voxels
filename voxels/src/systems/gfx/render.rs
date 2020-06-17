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
            // resources not loaded yet
            if res.is_none()
            {
                return;
            }
            let res = res.as_ref().unwrap();

            // go through every renderer, though there should
            // only be one
            for mut ctx in query.iter_mut(world)
            {
                // output frame
                let mut frame = ctx.frame();

                // <frame>
                {
                    // render pass
                    let mut pass = ctx.render_pass(&mut frame, [0.1, 0.2, 0.3, 1.0]);

                    // material, geometry
                    pass.pipeline(&res.pipeline);
                    pass.geometry(&res.geo);

                    // draw one instance
                    pass.draw(0..1);
                }
                // </frame>

                ctx.submit(&mut frame);
            }
        })
}