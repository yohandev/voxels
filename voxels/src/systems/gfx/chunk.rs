use ezgame::plugins::ezgfx::resources::*;
use ezgame::legion::*;
use ezmath::*;

use crate::components::game::*;
use crate::resources::game::*;
use crate::components::gfx::*;
use crate::game::*;

pub fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("chunk_mesh_system")
        // resources
        .read_resource::<Renderer>()
        .read_resource::<LoadedChunks>()
        // components
        .with_query(<Read<Chunk>>::query().filter(tag::<ChunkRemeshTag>()))
        .write_component::<ChunkMesh>()
        // tags
        .write_component::<ChunkRemeshTag>()
        // system
        .build(|cmd, world, (ctx, loaded), query|
        {
            if ctx.is_none()
            {
                return;
            }
            let ctx = ctx.as_ref().unwrap();

            for (ent, chunk) in query.iter_entities(world)
            {
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
                            let block = chunk[(x, y, z)];

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

                // no empty meshes(this crashes anyways)
                if vertices.is_empty()
                {
                    return;
                }

                // create mesh
                let mesh = ChunkMesh
                {
                    geo: ctx.geometry(&vertices[..], &indices[..])
                };

                // assign mesh component
                cmd.add_component(ent, mesh);
                println!("remeshed chunk!");
                    
                // done meshing, remove tag
                cmd.remove_tag::<ChunkRemeshTag>(ent);
            }
        })
}

fn gen_face(verts: &mut Vec<ChunkVertex>, ind: &mut Vec<u32>, n: usize, pos: uint3)
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

    debug_assert!(n < 6, "direction index cannot be n >= 6");

    for i in &TRI[n]            // vertices
    {
        let x = POS[*i][0] + pos.x;
        let y = POS[*i][1] + pos.y;
        let z = POS[*i][2] + pos.z;

        verts.push(ChunkVertex::new(&uint3::new(x, y, z), &uint2::new(pos.x, pos.z)));
        //buf.nor.push(Normal(NOR[ n]));
    }

    let j = verts.len() as u32;
    for i in &IND               // indices
    {
        ind.push(*i + j);
    }
}

fn dir(n: usize) -> int3
{
    const DIR: [[i32; 3]; 6] =
    [
        [  0,  0,  1  ],
        [  1,  0,  0  ],
        [  0,  0, -1  ],
        [ -1,  0,  0  ],
        [  0,  1,  0  ],
        [  0, -1,  0  ],
    ];

    int3::new(DIR[n][0], DIR[n][1], DIR[n][2])
}