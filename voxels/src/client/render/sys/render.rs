use ezgame::time::evt;
use ezgame::ecs::*;
use ezgame::gfx::*;

/// system that renders 3D models
pub struct SRender;

impl System for SRender
{
    const EVENT: Event = evt::RENDER;
    const ORDER: Order = ord::MID;

    const FLUSH: bool = true;

    fn exe() -> SysFn
    {
        // begin...
        sys("render_system")
        // components...
        .with_query(<(Read<crate::shared::chunk::CChunk>, Read<crate::client::ChunkMesh>)>::query())
        // resources...
        .read_resource::<crate::client::RGraphicsShared>()
        .read_resource::<crate::client::RGraphicsChunk>()
        .write_resource::<RGraphics>()
        // system...
        .build(|_, world, (r_shared, r_chunk, r_gfx), q_chunks|
        {
            if r_gfx.is_none()
            {
                return;
            }
            let gfx = r_gfx.as_mut().unwrap();

            // get frame
            let mut frame = gfx.frame();

            // <frame>
            {
                // render pass
                let mut pass = gfx.render_pass(&mut frame, [0.1, 0.2, 0.3, 1.0]);

                // shared
                if let Some(shared) = &**r_shared
                {
                    // uniforms
                    pass.bind_group(0, &shared.0);
                }

                // chunks
                if let Some(chunk) = &**r_chunk
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
            gfx.submit(&mut frame);
        })
    }
}