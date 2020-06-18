use ezgame::plugins::ezgfx::resources::*;
use ezgame::legion::*;

use crate::components::gfx::*;
use crate::resources::gfx::*;

pub(super) fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("render_system")
        // components
        .with_query(<Read<Camera>>::query())
        // resources
        .write_resource::<Renderer>()
        .write_resource::<SimpleGfxResources>()
        // system
        .build(|_, _, (ctx, res), _|
        {
            // resources not loaded yet
            if res.is_none()
            {
                return;
            }
            let res = res.as_ref().unwrap();

            let ctx = ctx.as_mut().unwrap();
            
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
        })
}