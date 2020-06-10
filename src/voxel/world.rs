use std::collections::HashMap;
use std::ops::*;

use crate::raytracer::*;
use super::*;

pub struct World
{
    /// read-only seed
    seed: u32,

    /// all currently loaded chunks
    chunks: HashMap<int3, Box<Chunk>>,

    /// dummy air block to return when falsy indexing
    air: Block,
}

impl World
{
    pub fn new(seed: u32) -> Self
    {
        let mut world = Self
        {
            seed,
            chunks: Default::default(),
            air: Default::default()
        };
        world.load(int3::new(0, 0, -80));

        world
    }

    pub fn seed(&self) -> u32
    {
        self.seed
    }

    pub fn load(&mut self, mut chunk_pos: int3)
    {
        chunk_pos.x -= chunk_pos.x % CHUNK_SIZE as i32;
        chunk_pos.y -= chunk_pos.y % CHUNK_SIZE as i32;
        chunk_pos.z -= chunk_pos.z % CHUNK_SIZE as i32;

        self.chunks.insert(chunk_pos, Box::new(Chunk::load(&self, chunk_pos)));
    }
}

impl Index<int3> for World
{
    type Output = Block;

    fn index(&self, index: int3) -> &Self::Output
    {
        let rx = index.x.rem_euclid(CHUNK_SIZE as i32);
        let ry = index.y.rem_euclid(CHUNK_SIZE as i32);
        let rz = index.z.rem_euclid(CHUNK_SIZE as i32);

        let x = index.x - rx;
        let y = index.y - ry;
        let z = index.z - rz;

        if let Some(chunk) = self.chunks.get(&int3::new(x, y, z))
        {
            &chunk[(rx, ry, rz)]
        }
        else
        {
            // println!
            // (
            //     "[warn] {}<{}, {}, {}>. {}. {}.",
            //     "attempting to index unloaded chunk! @ ",
            //     index.x, index.y, index.z,
            //     "returned air block instead",
            //     "use mutable indexer to load chunk if needed"
            // );

            &self.air
        }
    }
}

impl IndexMut<int3> for World
{
    fn index_mut(&mut self, index: int3) -> &mut Self::Output
    {
        let rx = index.x.rem_euclid(CHUNK_SIZE as i32);
        let ry = index.y.rem_euclid(CHUNK_SIZE as i32);
        let rz = index.z.rem_euclid(CHUNK_SIZE as i32);

        let x = index.x - rx;
        let y = index.y - ry;
        let z = index.z - rz;

        let pos = int3::new(x, y, z);

        if !self.chunks.contains_key(&pos)
        {
            println!
            (
                "[info] {}<{}, {}, {}>. {}",
                "mutably indexing unloaded chunk @ ",
                index.x, index.y, index.z,
                "loading chunk...",
            );
            //self.chunks.insert(pos, Box::new(Chunk::load(&self, pos)));
            self.load(pos);
        }

        self.chunks
            .get_mut(&pos)
            .unwrap()
            .index_mut((rx, ry, rz))
    }
}

impl Index<(i32, i32, i32)> for World
{
    type Output = Block;

    fn index(&self, index: (i32, i32, i32)) -> &Self::Output
    {
        &self[int3::new(index.0, index.1, index.2)]
    }
}

impl IndexMut<(i32, i32, i32)> for World
{
    fn index_mut(&mut self, index: (i32, i32, i32)) -> &mut Self::Output
    {
        &mut self[int3::new(index.0, index.1, index.2)]
    }
}

impl Renderable for World
{
    fn material(&self) -> &Material
    {
        const RED_RUBBER: Material = Material
        {
            diffuse: [77, 26, 26],
            albedo: [0.9, 0.1],
            specular: 10.0,
            reflectivity: 0.0,
            refractivity: 0.0,
            ior: 1.0,  
        };

        &RED_RUBBER
    }

    fn hits<'a>(&'a self, ray: &'a Ray) -> Option<RayCastHit<'a>>
    {
        const MAX_RAY_DIST: f32 = 64.0;

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