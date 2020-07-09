use ezgame::time::evt;
use ezgame::ecs::*;
use ezgame::*;

use crate::client::gfx::
{
    RGraphicsChunk,
    RGraphicsShared,
};

/// system that renders 3D models
pub struct SRender;

impl System for SRender
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<evt::Render>(0, Self::on_render);
    }
}

impl SRender
{
    fn on_render(app: &mut Application)
    { 
        // get graphics context
        if app.gfx().is_none()
        {
            return;
        }
        let ctx = app.gfx_mut().unwrap();

        // get frame
        let mut frame = ctx.frame();

        // <frame>
        {
            // render pass
            let mut pass = ctx.render_pass(&mut frame, [0.1, 0.2, 0.3, 1.0]);

            // shared graphic resources
            if let Some(shared) = &*app
                .res()
                .get::<RGraphicsShared>()
                .unwrap()
            {
                // view projection uniform
                pass.bind_group(0, &shared.0);
            }

            // chunks graphic resources
            if let Some(chunk) = &*app
                .res()
                .get::<RGraphicsChunk>()
                .unwrap()
            {
                // pipeline
                pass.pipeline(&chunk.3);

                // iter meshes
                for mesh in chunk.4.values()
                {
                    pass.bind_group(1, &mesh.pos);
                    pass.geometry(&mesh.geo);
                    pass.draw(0..1);
                }
            }
        }
        // </frame>

        // submit pass
        ctx.submit(&mut frame);
    }
}