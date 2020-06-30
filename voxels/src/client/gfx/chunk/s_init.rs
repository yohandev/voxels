use ezgame::ecs::*;
use ezgame::gfx::*;

use super::super::
{
    ChunkPosition, 
    ChunkVertex, 
    RGraphicsShared, 
    RGraphicsChunk,
    SGraphicsShared,
};

/// system that initializes the RGraphicsChunk
/// resource
pub struct SGraphicsChunk;

impl System for SGraphicsChunk
{
    const EVENT: Event = evt::READY;
    const ORDER: Order = SGraphicsShared::ORDER + 1;

    fn prepare(r: &mut Resources)
    {
        r.insert(RGraphicsChunk::None);
    }

    fn exe() -> SysFn
    {
        // begin...
        sys("chunk_graphics_init_system")
        // resources
        .read_resource::<RGraphics>()
        .read_resource::<RGraphicsShared>()
        .write_resource::<RGraphicsChunk>()
        // system
        .build(move |_, _, (r_gfx, r_shared, r_chunk), _|
        {
            const VS_SRC: &str = include_str!("../../../../assets/shaders/chunk.vert");
            const FS_SRC: &str = include_str!("../../../../assets/shaders/chunk.frag");

            let ctx = r_gfx.as_ref().unwrap();
            
            let vs = ctx.shader(ShaderKind::Vertex, VS_SRC);
            let fs = ctx.shader(ShaderKind::Fragment, FS_SRC);

            let vp = &r_shared.as_ref().unwrap().0;

            let pos = ctx.uniform(ChunkPosition::default());
            let pos = ctx.bind_group(ShaderKind::Vertex, (pos,));

            let pipeline = ctx
                .pipeline()
                    .bindings(&[vp, &pos])
                    .vertex::<ChunkVertex>()
                    .index::<u32>()
                    .shader(&vs)
                    .shader(&fs)
                    .depth(true)
                .build();
            
            r_chunk.replace((vs, fs, pos, pipeline, Default::default()));
        })
    }
}