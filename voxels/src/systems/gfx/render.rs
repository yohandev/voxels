use ezgame::plugins::ezgfx::resources::*;
use ezgame::legion::*;
use ezmath::*;

use crate::components::transform::LocalToWorld;
use crate::components::gfx::*;
use crate::resources::gfx::*;

pub(super) fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("render_system")
        // components
        .with_query
        (
            <(Read<Camera>, TryRead<LocalToWorld>)>::query()
                .filter
                (
                    changed::<Camera>() |
                    changed::<LocalToWorld>()
                )
        )
        // resources
        .write_resource::<Renderer>()
        .write_resource::<SimpleGfxResources>()
        // system
        .build(|_, world, (ctx, res), query|
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

            // get first camera
            for (cam, ltw) in query.iter(world)
            {
                // (view) projection matrix
                let vp = if let Some(ltw) = &ltw { cam.proj * ltw.0.inverse() } else { cam.proj };

                // update uniforms
                ctx.update_uniform(&res.vp.bindings.0, ViewProjUniform::new(vp));

                // break after first camera
                break;
            }

            // <frame>
            {
                // render pass
                let mut pass = ctx.render_pass(&mut frame, [0.1, 0.2, 0.3, 1.0]);

                // material, geometry
                pass.pipeline(&res.pipeline);
                pass.geometry(&res.geo);

                // uniforms
                pass.bind_group(0, &res.vp);

                // draw one instance
                pass.draw(0..1);
            }
            // </frame>

            ctx.submit(&mut frame);
        })
}