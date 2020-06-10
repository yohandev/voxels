use std::ops::*;

use crate::raytracer::*;
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
    pos: int3
}

impl Chunk
{
    /// load a chunk
    pub fn load(world: &World, pos: int3) -> Self
    {
        Self
        {
            blocks: [Block::default(); CHUNK_VOLUME],
            pos
        }
    }

    /// flatten a relative position index to a 1D array index
    fn flat_index(r_pos: &int3) -> usize
    {
        (r_pos.x as usize              ) +
        (r_pos.y as usize * CHUNK_SIZE ) +
        (r_pos.z as usize * CHUNK_LAYER)
    }

    fn flat_index_tuple(r_pos: &(i32, i32, i32)) -> usize
    {
        (r_pos.0 as usize              ) +
        (r_pos.1 as usize * CHUNK_SIZE ) +
        (r_pos.2 as usize * CHUNK_LAYER)
    }
}

impl Renderable for Chunk
{
    fn material(&self) -> &Material
    {
        const MATERIAL: Material = Material
        {
            diffuse: [77, 26, 26],
            albedo: [0.9, 0.1],
            specular: 10.0,
            reflectivity: 0.0,
            refractivity: 0.0,
            ior: 1.0,
        };

        &MATERIAL
    }

    fn hits<'a>(&'a self, ray: &'a Ray) -> Option<RayCastHit<'a>>
    {
        const MAX_RAY_DIST: f32 = 128.0;

        let mut t = 0.0;

        let mut pt = int3::new
        (
            ray.origin.x.floor() as i32,
            ray.origin.y.floor() as i32,
            ray.origin.z.floor() as i32,
        );

        let step = int3::new
        (
            if ray.direct.x > 0.0 { 1 } else { -1 },
            if ray.direct.y > 0.0 { 1 } else { -1 },
            if ray.direct.z > 0.0 { 1 } else { -1 },
        );

        let delta = float3::new
        (
            (1.0 / ray.direct.x).abs(),
            (1.0 / ray.direct.y).abs(),
            (1.0 / ray.direct.z).abs(),
        );

        let dist = float3::new
        (
            if step.x > 0 { pt.x as f32 + 1.0 - ray.origin.x } else { ray.origin.x - pt.x as f32 },
            if step.x > 0 { pt.y as f32 + 1.0 - ray.origin.y } else { ray.origin.y - pt.y as f32 },
            if step.x > 0 { pt.z as f32 + 1.0 - ray.origin.z } else { ray.origin.z - pt.z as f32 },
        );

        let mut max = float3::new
        (
            if delta.x.is_finite() { delta.x * dist.x } else { f32::INFINITY },
            if delta.y.is_finite() { delta.y * dist.y } else { f32::INFINITY },
            if delta.z.is_finite() { delta.z * dist.z } else { f32::INFINITY },
        );

        let mut step_index = -1;

        while t < MAX_RAY_DIST
        {
            let b = self[pt];
            if !b.is_air()
            {
                return Some
                (
                    crate::raytracer::RayCastHit
                    {
                        distance: t,
                        point: ray.origin + (t * delta),
                        normal: float3::new
                        (
                            if step_index == 0 { -step.x as f32 } else { 0.0 },
                            if step_index == 1 { -step.y as f32 } else { 0.0 },
                            if step_index == 2 { -step.z as f32 } else { 0.0 },
                        ),
                        ray: &ray,
                        collide: self,
                    }
                )
            }

            if max.x < max.y
            {
                if max.x < max.z
                {
                    pt.x += step.x;
                    t = max.x;
                    max.x += delta.x;
                    step_index = 0;
                }
                else
                {
                    pt.z += step.z;
                    t = max.z;
                    max.z += delta.z;
                    step_index = 2;
                }
            }
            else
            {
                if max.y < max.z
                {
                    pt.y += step.y;
                    t = max.y;
                    max.y += delta.y;
                    step_index = 1;
                }
                else
                {
                    pt.z += step.z;
                    t = max.z;
                    max.z += delta.z;
                    step_index = 2;
                }
            }
        }
        None
    }
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

impl Index<(i32, i32, i32)> for Chunk
{
    type Output = Block;

    /// get a block within this chunk, given a relative position
    fn index(&self, index: (i32, i32, i32)) -> &Self::Output
    {
        &self.blocks[Self::flat_index_tuple(&index)]
    }
}

impl IndexMut<(i32, i32, i32)> for Chunk
{
    /// get a block within this chunk, given a relative position
    fn index_mut(&mut self, index: (i32, i32, i32)) -> &mut Self::Output
    {
        &mut self.blocks[Self::flat_index_tuple(&index)]
    }
}