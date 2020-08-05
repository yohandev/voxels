use ezgame::ecs::*;
use ezgame::gfx::*;
use ezgame::time;
use ezmath::*;

use crate::common::chunk::{ CChunk, CBlockBuffer, BlockBufferIndex, TUpdated, RChunkCache };
use crate::client::gfx::{ SRender, RGraphicsChunk, ChunkMeshBuilder, ChunkPosition, ChunkMesh };
use crate::common::block::{ Block, BlockFace, shapes::BlockShapes, RBlockPalette };
use crate::common::CHUNK_SIZE;

/// system that remeshes chunks
pub struct SChunkMesh;

impl System for SChunkMesh
{
    const EVENT: Event = time::evt::RENDER;
    const ORDER: Order = SRender::ORDER - 1;

    const FLUSH: bool = true;

    fn exe() -> Job
    {
        // begin...
        sys("chunk_mesh_system")
        // components...
        .with_query(<Read<CChunk>>::query().filter(tag::<TUpdated>()))
        .read_component::<CBlockBuffer>()
        // resources...
        .read_resource::<RChunkCache>()
        .read_resource::<RBlockPalette>()
        .read_resource::<RGraphics>()
        .write_resource::<RGraphicsChunk>()
        // system...
        .build(|cmd, world, (r_cache, r_pal, r_gfx, r_gfx_chunk), q_chunk|
        {
            if r_gfx.is_none() || r_gfx_chunk.is_none()
            {
                return;
            }
            let gfx = r_gfx.as_ref().unwrap();
            let gfx_chunk = r_gfx_chunk.as_mut().unwrap();

            for (ent, chunk) in q_chunk.iter_entities(world)
            {
                // neighbors
                let region = Region::new(chunk.pos(), world, r_cache, r_pal);

                // geometry buffer
                let mut mesh = ChunkMeshBuilder::default();

                // go through every block
                for x in 0..CHUNK_SIZE as i32
                {
                    for y in 0..CHUNK_SIZE as i32
                    {
                        for z in 0..CHUNK_SIZE as i32
                        {
                            // target position
                            let pos = int3::new(x, y, z);

                            // target block
                            let block = region.center.get(pos, region.pal);

                            // ignore air
                            if block.shape() == BlockShapes::None
                            {
                                continue;
                            }

                            // check block in every direction
                            for d in 0..6usize
                            {
                                let face = BlockFace::from(d);

                                // only generate face is neighbor face isn't
                                // full opaque
                                if !region.culled(&block, face)
                                {
                                    block.mesh(&mut mesh, face);
                                }
                            }
                        }
                    }
                }

                // done meshing, remove tag
                cmd.remove_tag::<TUpdated>(ent);

                // no empty meshes(this crashes anyways)
                if mesh.vert.is_empty()
                {
                    continue;
                }

                // position uniform
                let pos = gfx.uniform(ChunkPosition { position: chunk.pos() });

                //create mesh
                let mesh = ChunkMesh
                {
                    geo: gfx.geometry(&mesh.vert[..], &mesh.ind[..]),
                    pos: gfx.clone_bind_group(&gfx_chunk.2, (pos,))
                };

                //push mesh
                gfx_chunk.4.insert(chunk.pos(), mesh);

                println!("remeshed chunk!");
            }
        })
    }
}

/// represents a chunk and its cached neighbors
struct Region<'a>
{
    center: CmpRef<'a, CBlockBuffer>,
    neighbors: [Option<CmpRef<'a, CBlockBuffer>>; 6],

    pal: &'a RBlockPalette
}

impl<'a> Region<'a>
{
    fn new(center: int3, world: &'a SubRegistry, cache: &'a RChunkCache, pal: &'a RBlockPalette) -> Self
    {
        // neighbors
        let mut neighbors = [None, None, None, None, None, None];
        for i in 0..6usize
        {
            let dir = BlockFace::from(i).normal() * CHUNK_SIZE as i32;

            if let Some(ent) = cache.at(center + dir)
            {
                neighbors[i] = world.get_component::<CBlockBuffer>(*ent);
            }
        }

        // center
        let center = world
            .get_component::<CBlockBuffer>(*cache.at(center).unwrap())
            .unwrap();

        Self
        {
            center,
            neighbors,
            pal
        }
    }

    /// get a block in this region given the relative
    /// coordinates.
    fn culled(&self, block: &Block, face: BlockFace) -> bool
    {
        const SIZE: i32 = CHUNK_SIZE as i32;

        // neighbor block pos global
        let n_pos = block.r_pos() + face.normal();

        if n_pos.x == -1 || n_pos.x == SIZE
        || n_pos.y == -1 || n_pos.y == SIZE
        || n_pos.z == -1 || n_pos.z == SIZE
        {
            // neighbor block ros relative
            let rx = n_pos.x.rem_euclid(SIZE);
            let ry = n_pos.y.rem_euclid(SIZE);
            let rz = n_pos.z.rem_euclid(SIZE);

            // neighbor chunk
            if let Some(neighbor) = &self.neighbors[face as usize]
            {
                // do test
                block.cull(&neighbor.get((rx, ry, rz), self.pal), face)
            }
            else { false }
        }
        else
        {
            // do test
            block.cull(&self.center.get(n_pos, self.pal), face)
        }
    }
}