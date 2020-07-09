use ezgame::ecs::*;
use ezgame::gfx::*;
use ezgame::*;

use super::super::
{
    ChunkPosition, 
    ChunkVertex, 
    RGraphicsShared, 
    RGraphicsChunk,
};

/// system that initializes the RGraphicsChunk
/// resource
pub struct SGraphicsChunk;

impl System for SGraphicsChunk
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<ezgame::evt::Start>(-999, Self::on_start);
        handlers.insert::<gfx::evt::Ready>(-995, Self::on_ezgfx_ready);
    }
}

impl SGraphicsChunk
{
    fn on_start(app: &mut Application)
    {
        app.resources().insert(RGraphicsChunk::None);
    }

    fn on_ezgfx_ready(app: &mut Application)
    {
        if let Some(ctx) = &*app.gfx()
        {
            // shaders source
            const VS_SRC: &str = include_str!("../../../../assets/shaders/chunk.vert");
            const FS_SRC: &str = include_str!("../../../../assets/shaders/chunk.frag");
            
            // compiled shaders
            let vs = ctx.shader(ShaderKind::Vertex, VS_SRC);
            let fs = ctx.shader(ShaderKind::Fragment, FS_SRC);

            // shared view-projection bind group
            let vp = &app
                .res()
                .get::<RGraphicsShared>()
                .unwrap()
                .as_ref()
                .unwrap().0;

            // chunk position uniform
            let pos = ctx.uniform(ChunkPosition::default());
            // chunk position bind group
            let pos = ctx.bind_group(ShaderKind::Vertex, (pos,));

            // chunk pipeline
            let pipeline = ctx
                .pipeline()
                    .bindings(&[vp, &pos])
                    .vertex::<ChunkVertex>()
                    .index::<u32>()
                    .shader(&vs)
                    .shader(&fs)
                    .depth(true)
                .build();
            
            // explicit typing to make sure type matches
            let res: RGraphicsChunk = Some((vs, fs, pos, pipeline, Default::default()));

            // replace resource
            app.resources().insert(res);
        }
    }
}