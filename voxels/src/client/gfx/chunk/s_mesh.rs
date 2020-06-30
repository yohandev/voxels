use ezgame::ecs::*;
use ezgame::gfx::*;
use ezgame::time;
use ezmath::*;

use crate::client::gfx::{ SRender, RGraphicsChunk, ChunkVertex, ChunkPosition, ChunkMesh };
use crate::common::chunk::{ CChunk, CBlockBuffer, TUpdated };
use crate::common::{ CHUNK_SIZE, Direction, Block };

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
            <(Read<CChunk>, Read<CBlockBuffer>)>::query().filter(tag::<TUpdated>())
        )
        .read_component::<CBlockBuffer>()
        // resources...
        .read_resource::<RGraphics>()
        .write_resource::<RGraphicsChunk>()
        // system...
        .build(|cmd, world, (r_gfx, r_gfx_chunk), q_chunk|
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
                let region = Region::new((&chunk, &blocks), world);

                // geometry buffer
                let mut vertices = Vec::<ChunkVertex>::new();
                let mut indices  = Vec::<u32>::new();

                // go through every block
                for x in 0..CHUNK_SIZE as i32
                {
                    for y in 0..CHUNK_SIZE as i32
                    {
                        for z in 0..CHUNK_SIZE as i32
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
                                let dir: int3 = Direction::from(d).into();

                                let dx = dir.x;
                                let dy = dir.y;
                                let dz = dir.z;

                                let neighbor = region.get(x + dx, y + dy, z + dz);

                                // only generate face is neighbor face isn't
                                // full opaque
                                if neighbor.is_none() || neighbor.unwrap().is_air()
                                {
                                    gen_face(&mut vertices, &mut indices, d, int3::new(x, y, z));
                                }
                            }
                        }
                    }
                }

                // no empty meshes(this crashes anyways)
                if vertices.is_empty()
                {
                    return;
                }

                // position uniform
                let pos = gfx.uniform(ChunkPosition { position: chunk.pos() });

                //create mesh
                let mesh = ChunkMesh
                {
                    geo: gfx.geometry(&vertices[..], &indices[..]),
                    pos: gfx.clone_bind_group(&gfx_chunk.2, (pos,))
                };

                //push mesh
                gfx_chunk.4.insert(chunk.pos(), mesh);
                
                println!("remeshed chunk!");
                
                // done meshing, remove tag
                cmd.remove_tag::<TUpdated>(ent);
            }
        })
    }
}

/// represents a chunk and its cached neighbors
struct Region<'a>
{
    center: &'a CBlockBuffer,
    neighbors: [Option<CmpRef<'a, CBlockBuffer>>; 6]
}

impl<'a> Region<'a>
{
    fn new(center: (&'a CChunk, &'a CBlockBuffer), world: &'a SubRegistry) -> Self
    {
        // neighbors
        let mut neighbors = [None, None, None, None, None, None];

        // iter chunks
        for (ent, chunk) in <Read<CChunk>>::query().iter_entities(world)
        {
            // iter directions
            for i in 0..6usize
            {
                let mut dir: int3 = Direction::from(i).into();

                dir *= CHUNK_SIZE as i32;

                if chunk.pos() == center.0.pos() + dir
                {
                    neighbors[i] = world.get_component::<CBlockBuffer>(ent);
                }
            }
        }
        
        Self
        {
            center: center.1,
            neighbors,
        }
    }

    /// get a block in this region given the relative
    /// coordinates. returns None is the block isn't
    /// loaded.
    fn get(&self, x: i32, y: i32, z: i32) -> Option<Block>
    {
        const SIZE: i32 = CHUNK_SIZE as i32;

        if x < 0 || x >= SIZE
        || y < 0 || y >= SIZE
        || z < 0 || z >= SIZE
        {
            use std::convert::TryFrom;

            let rx = x.rem_euclid(SIZE);
            let ry = y.rem_euclid(SIZE);
            let rz = z.rem_euclid(SIZE);

            if let Ok(dir) = Direction::try_from((x, y, z))
            {
                let i: usize = dir.into();

                if let Some(neighbor) = &self.neighbors[i]
                {
                    Some(neighbor[(rx, ry, rz)])
                } else { None }
            } else { None }
        }
        else
        {
            Some(self.center[(x, y, z)])
        }
    }
}

fn gen_face(verts: &mut Vec<ChunkVertex>, ind: &mut Vec<u32>, dir: usize, pos: int3)
{
    const POS: [[u32; 3]; 8] = 
    [
        [ 1 , 1 , 1 ],
        [ 0 , 1 , 1 ],
        [ 0 , 0 , 1 ],
        [ 1 , 0 , 1 ],
        [ 0 , 1 , 0 ],
        [ 1 , 1 , 0 ],
        [ 1 , 0 , 0 ],
        [ 0 , 0 , 0 ],
    ];

    // const NOR: [[f32; 3]; 6] =
    // [
    //     [ 0.0, 0.0, 1.0 ],
    //     [ 1.0, 0.0, 0.0 ],
    //     [ 0.0, 0.0, -1. ],
    //     [ -1., 0.0, 0.0 ],
    //     [ 0.0, 1.0, 0.0 ],
    //     [ 0.0, -1., 0.0 ],
    // ];

    const TRI: [[usize; 4]; 6] =
    [
        [ 0, 1, 2, 3 ],
        [ 5, 0, 3, 6 ],
        [ 4, 5, 6, 7 ],
        [ 1, 4, 7, 2 ],
        [ 5, 4, 1, 0 ],
        [ 3, 2, 7, 6 ],
    ];

    const IND: [u32; 6] =
    [
        0, 1, 2, 0, 2, 3
    ];

    for i in &TRI[dir]            // vertices
    {
        let x = POS[*i][0] + pos.x as u32;
        let y = POS[*i][1] + pos.y as u32;
        let z = POS[*i][2] + pos.z as u32;

        verts.push(ChunkVertex::new(&uint3::new(x, y, z), &uint2::new(pos.x as u32, pos.z as u32)));
        //buf.nor.push(Normal(NOR[ n]));
    }

    let j = verts.len() as u32;
    for i in &IND               // indices
    {
        ind.push(*i + j);
    }
}