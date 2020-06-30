use ezgame::ecs::*;
use ezgame::gfx::*;
use ezgame::time;

use crate::client::gfx::{ SRender, RGraphicsChunk, ChunkVertex };
use crate::common::chunk::{ CChunk, CBlockBuffer, TUpdated };
use crate::common::CHUNK_SIZE;

/// system that remeshes chunks
pub struct SChunkMesh;

impl System for SChunkMesh
{
    const EVENT: Event = time::evt::RENDER;
    const ORDER: Order = SRender::ORDER - 1;

    const FLUSH: bool = true;

    fn exe() -> SysFn
    {
        // begin...
        sys("chunk_mesh_system")
        // components...
        .with_query
        (
            <(Read<CChunk>, Read<CBlockBuffer>)>::query()
                .filter(tag::<TUpdated>())
        )
        // resources...
        .read_resource::<RGraphics>()
        .write_resource::<RGraphicsChunk>()
        // system...
        .build(|_, world, (r_gfx, r_gfx_chunk), q_chunk|
        {
            if r_gfx.is_none() || r_gfx_chunk.is_none()
            {
                return;
            }
            let gfx = r_gfx.as_ref().unwrap();
            let gfx_chunk = r_gfx_chunk.as_mut().unwrap();

            for (ent, (chunk, blocks)) in q_chunk.iter_entities(world)
            {
                // neighbors
                let mut neighbors = [Option::<CBlockBuffer>::None; 6];

                // geometry buffer
                let mut vertices = Vec::<ChunkVertex>::new();
                let mut indices = Vec::<u32>::new();

                // go through every block
                for x in 0..CHUNK_SIZE as u32
                {
                    for y in 0..CHUNK_SIZE as u32
                    {
                        for z in 0..CHUNK_SIZE as u32
                        {
                            // target block
                            let block = blocks[(x, y, z)];

                            // ignore air
                            if block.is_air()
                            {
                                continue;
                            }

                            // check block in every direction
                            for d in 0..6usize
                            {
                                let dir = dir(d);

                                // only generate face is neighbor face isn't
                                // full opaque
                                if loaded.get_block(chunk.position() + int3::new(x as i32, y as i32, z as i32) + dir, world).is_air()
                                {
                                    gen_face(&mut vertices, &mut indices, d, uint3::new(x, y, z));
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}