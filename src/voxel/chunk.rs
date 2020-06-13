use std::ops::*;

use crate::gfx::vertices::ChunkVertex;
use crate::gfx::mesh::ChunkMesh;
use crate::framework::RenderCtx;
use super::*;

pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_LAYER: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: usize = CHUNK_LAYER * CHUNK_SIZE;

pub struct Chunk
{
    /// raw blocks storage
    blocks: [Block; CHUNK_VOLUME],

    /// position of the min block in the chunk
    ///
    /// if the chunk spans from (0, 0, 0) to (32, 32, 32),
    /// pos would be (0, 0, 0).
    pos: int3,

    /// mesh of chunk
    pub mesh: Option<ChunkMesh>,
}

impl Chunk
{
    /// load a chunk
    pub(super) fn load(world: &Dimension, pos: int3) -> Self
    {
        Self
        {
            blocks: [Block::default(); CHUNK_VOLUME],
            mesh: None,
            pos,
        }
    }

    pub fn generate(&mut self, world: &Dimension)
    {
        use noise::*;

        const SEA_LEVEL: f64 = 10.0;
        const TERRAIN_DELTA: f64 = 10.0;

        let perlin = Perlin::new().set_seed(world.seed());

        for rx in 0..CHUNK_SIZE as u32
        {
            for rz in 0..CHUNK_SIZE as u32
            {
                let x = (rx as i32 + self.pos.x) as f64;
                let z = (rz as i32 + self.pos.z) as f64;

                let h = (perlin.get([x / 15.0, z / 15.0]) * TERRAIN_DELTA + SEA_LEVEL) as i32;
                let rh = h - self.pos.y;

                for ry in 0..rh.min(CHUNK_SIZE as i32)
                {
                    self[(rx, ry as u32, rz)] = Block::new(0b0000_0000_0000_1000);
                }
            }
        }
    }

    pub(super) fn remesh(&self, ctx: &RenderCtx, world: &Dimension/*, neighbors: [&Chunk; 6]*/) -> Option<ChunkMesh>
    {
        let mut vertices = Vec::<ChunkVertex>::new();
        let mut indices = Vec::<u32>::new();

        // vertices.push(ChunkVertex::new(&uint3::new(0, 0, 0), &uint2::new(0, 0)));
        // vertices.push(ChunkVertex::new(&uint3::new(0, 1, 0), &uint2::new(0, 0)));
        // vertices.push(ChunkVertex::new(&uint3::new(1, 0, 0), &uint2::new(0, 0)));

        // indices.push(0);
        // indices.push(2);
        // indices.push(1);

        for x in 0..CHUNK_SIZE as u32
        {
            for y in 0..CHUNK_SIZE as u32
            {
                for z in 0..CHUNK_SIZE as u32
                {
                    let block = self[(x, y, z)];
                    if block.is_air()
                    {
                        continue;
                    }

                    for d in 0..6usize
                    {
                        let dir = dir(d);

                        if world[self.pos + int3::new(x as i32, y as i32, z as i32) + dir].is_air()
                        {
                            gen_face(&mut vertices, &mut indices, d, uint3::new(x, y, z));
                        }
                    }
                }
            }
        }
        if vertices.is_empty()
        {
            None
        }
        else
        {
            Some(ChunkMesh::create(ctx, &vertices[..], &indices[..]))
        }
    }

    pub fn pos(&self) -> &int3
    {
        &self.pos
    }

    pub fn mesh(&self) -> &Option<ChunkMesh>
    {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Option<ChunkMesh>
    {
        &mut self.mesh
    }

    /// flatten a relative position index to a 1D array index
    fn flat_index(r_pos: &int3) -> usize
    {
        (r_pos.x as usize              ) +
        (r_pos.y as usize * CHUNK_SIZE ) +
        (r_pos.z as usize * CHUNK_LAYER)
    }

    fn flat_index_tuple(r_pos: &(u32, u32, u32)) -> usize
    {
        (r_pos.0 as usize              ) +
        (r_pos.1 as usize * CHUNK_SIZE ) +
        (r_pos.2 as usize * CHUNK_LAYER)
    }

    // fn hits<'a>(&'a self, ray: &'a Ray) -> Option<RayCastHit<'a>>
    // {
    //     const MAX_RAY_DIST: f32 = 128.0;

    //     let mut t = 0.0;

    //     let mut pt = int3::new
    //     (
    //         ray.origin.x.floor() as i32,
    //         ray.origin.y.floor() as i32,
    //         ray.origin.z.floor() as i32,
    //     );

    //     let step = int3::new
    //     (
    //         if ray.direct.x > 0.0 { 1 } else { -1 },
    //         if ray.direct.y > 0.0 { 1 } else { -1 },
    //         if ray.direct.z > 0.0 { 1 } else { -1 },
    //     );

    //     let delta = float3::new
    //     (
    //         (1.0 / ray.direct.x).abs(),
    //         (1.0 / ray.direct.y).abs(),
    //         (1.0 / ray.direct.z).abs(),
    //     );

    //     let dist = float3::new
    //     (
    //         if step.x > 0 { pt.x as f32 + 1.0 - ray.origin.x } else { ray.origin.x - pt.x as f32 },
    //         if step.x > 0 { pt.y as f32 + 1.0 - ray.origin.y } else { ray.origin.y - pt.y as f32 },
    //         if step.x > 0 { pt.z as f32 + 1.0 - ray.origin.z } else { ray.origin.z - pt.z as f32 },
    //     );

    //     let mut max = float3::new
    //     (
    //         if delta.x.is_finite() { delta.x * dist.x } else { f32::INFINITY },
    //         if delta.y.is_finite() { delta.y * dist.y } else { f32::INFINITY },
    //         if delta.z.is_finite() { delta.z * dist.z } else { f32::INFINITY },
    //     );

    //     let mut step_index = -1;

    //     while t < MAX_RAY_DIST
    //     {
    //         let b = self[pt];
    //         if !b.is_air()
    //         {
    //             return Some
    //             (
    //                 crate::raytracer::RayCastHit
    //                 {
    //                     distance: t,
    //                     point: ray.origin + (t * delta),
    //                     normal: float3::new
    //                     (
    //                         if step_index == 0 { -step.x as f32 } else { 0.0 },
    //                         if step_index == 1 { -step.y as f32 } else { 0.0 },
    //                         if step_index == 2 { -step.z as f32 } else { 0.0 },
    //                     ),
    //                     ray: &ray,
    //                     collide: self,
    //                 }
    //             )
    //         }

    //         if max.x < max.y
    //         {
    //             if max.x < max.z
    //             {
    //                 pt.x += step.x;
    //                 t = max.x;
    //                 max.x += delta.x;
    //                 step_index = 0;
    //             }
    //             else
    //             {
    //                 pt.z += step.z;
    //                 t = max.z;
    //                 max.z += delta.z;
    //                 step_index = 2;
    //             }
    //         }
    //         else
    //         {
    //             if max.y < max.z
    //             {
    //                 pt.y += step.y;
    //                 t = max.y;
    //                 max.y += delta.y;
    //                 step_index = 1;
    //             }
    //             else
    //             {
    //                 pt.z += step.z;
    //                 t = max.z;
    //                 max.z += delta.z;
    //                 step_index = 2;
    //             }
    //         }
    //     }
    //     None
    // }
}

impl Index<int3> for Chunk
{
    type Output = Block;

    /// get a block within this chunk, given a relative position
    fn index(&self, index: int3) -> &Self::Output
    {
        &self.blocks[Self::flat_index(&index)]
    }
}

impl IndexMut<int3> for Chunk
{
    /// get a block within this chunk, given a relative position
    fn index_mut(&mut self, index: int3) -> &mut Self::Output
    {
        &mut self.blocks[Self::flat_index(&index)]
    }
}

impl Index<(u32, u32, u32)> for Chunk
{
    type Output = Block;

    /// get a block within this chunk, given a relative position
    fn index(&self, index: (u32, u32, u32)) -> &Self::Output
    {
        &self.blocks[Self::flat_index_tuple(&index)]
    }
}

impl IndexMut<(u32, u32, u32)> for Chunk
{
    /// get a block within this chunk, given a relative position
    fn index_mut(&mut self, index: (u32, u32, u32)) -> &mut Self::Output
    {
        &mut self.blocks[Self::flat_index_tuple(&index)]
    }
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

    const NOR: [[f32; 3]; 6] =
    [
        [ 0.0, 0.0, 1.0 ],
        [ 1.0, 0.0, 0.0 ],
        [ 0.0, 0.0, -1. ],
        [ -1., 0.0, 0.0 ],
        [ 0.0, 1.0, 0.0 ],
        [ 0.0, -1., 0.0 ],
    ];

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