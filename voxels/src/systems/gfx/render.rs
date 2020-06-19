use ezgame::plugins::ezgfx::resources::*;
use ezgame::legion::*;
use ezmath::*;

use crate::components::transform::LocalToWorld;
use crate::components::gfx::*;
use crate::resources::gfx::*;

pub(super) fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("render_system")
        // camera components
        .with_query
        (
            <(Read<Camera>, TryRead<LocalToWorld>)>::query()
                .filter
                (
                    changed::<Camera>() |
                    changed::<LocalToWorld>()
                )
        )
        // chunk components
        // .with_query(<Read<ChunkMesh>>::query())
        // .read_component::<ChunkMesh>()
        // resources
        .write_resource::<Renderer>()
        .write_resource::<SimpleGfxResources>()
        .read_resource::<ChunkGfxResources>()
        // system
        .build(|_, world, (ctx, global_res, chunk_res), query|
        {
            // resources not loaded yet
            if global_res.is_none()
            {
                return;
            }
            let global_res = global_res.as_ref().unwrap();

            let ctx = ctx.as_mut().unwrap();
            
            // output frame
            let mut frame = ctx.frame();

            // get first camera
            for (cam, ltw) in query.iter(world)
            {
                // (view) projection matrix
                let vp = if let Some(ltw) = &ltw { cam.proj * ltw.0.inverse() } else { cam.proj };

                // update uniforms
                ctx.update_uniform(&global_res.vp.bindings.0, ViewProjUniform::new(vp));

                // break after first camera
                break;
            }

            // <frame>
            {
                // render pass
                let mut pass = ctx.render_pass(&mut frame, [0.1, 0.2, 0.3, 1.0]);

                // material, geometry
                pass.pipeline(&global_res.pipeline);
                pass.geometry(&global_res.geo);

                // uniforms
                pass.bind_group(0, &global_res.vp);

                // draw one instance
                pass.draw(0..1);

                // chunk resources
                if chunk_res.is_none()
                {
                    return;
                }
                let chunk_res = chunk_res.as_ref().unwrap();

                //let chunks = chunk_query.components::<ChunkMesh, SubWorld>(world);
                // -- chunks --
                pass.pipeline(&chunk_res.pipeline);

                // for chunk in &(*chunks)
                // {
                //     println!("chunk in lol");

                //     pass.geometry(&chunk.geo);
                //     pass.draw(0..1);
                // }
                // for chunk in chunk_query.iter(world)
                // {
                //     pass.geometry(&chunk.geo);
                // }
                for mesh in &chunk_res.chunk_meshes
                {
                    pass.bind_group(1, &mesh.pos);
                    pass.geometry(&mesh.geo);
                    pass.draw(0..1);
                }
            }
            // </frame>

            ctx.submit(&mut frame);
        })
}