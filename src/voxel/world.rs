use std::collections::hash_map::*;
use std::ops::*;

use crate::framework::RenderCtx;
use crate::gfx::Gfx;
use super::*;

pub struct Dimension
{
    /// seed used to generate the world
    seed: u32,

    /// all currently loaded chunks
    chunks: HashMap<int3, Box<Chunk>>,

    /// ecs entities in this world
    ecs: EcsRegistry,

    /// dummy air block to return when falsy indexing
    air: Block,
}

impl Dimension
{
    pub fn new(registry: EcsRegistry) -> Self
    {
        Self
        {
            seed: 12345,
            chunks: Default::default(),
            ecs: registry,
            air: Block::default()
        }
    }

    pub fn load_chunk(&mut self, mut pos: int3)
    {
        pos.x -= pos.x % CHUNK_SIZE as i32;
        pos.y -= pos.y % CHUNK_SIZE as i32;
        pos.z -= pos.z % CHUNK_SIZE as i32;

        let mut chunk = Box::new(Chunk::load(&self, pos));

        chunk.generate(self);

        self.chunks.insert(pos, chunk);
    }

    pub fn remesh_chunk(&mut self, pos: int3, ctx: &RenderCtx, gfx: &Gfx)
    {
        // check neighbors
        // let px = self.chunks.get(&(pos + CHUNK_SIZE as i32 * int3::new(1, 0, 0)));
        // if px.is_none() { return; }
        // let nx = self.chunks.get(&(pos + CHUNK_SIZE as i32 * int3::new(-1, 0, 0)));
        // if nx.is_none() { return; }
        // let py = self.chunks.get(&(pos + CHUNK_SIZE as i32 * int3::new(0, 1, 0)));
        // if py.is_none() { return; }
        // let ny = self.chunks.get(&(pos + CHUNK_SIZE as i32 * int3::new(0, -1, 0)));
        // if ny.is_none() { return; }
        // let pz = self.chunks.get(&(pos + CHUNK_SIZE as i32 * int3::new(0, 0, 1)));
        // if pz.is_none() { return; }
        // let nz = self.chunks.get(&(pos + CHUNK_SIZE as i32 * int3::new(0, 0, -1)));
        // if nz.is_none() { return; }

        let gfx = self.chunks
            .get(&pos)
            .unwrap()
            .remesh(ctx, gfx, self);

        let chunk = self.chunks
            .get_mut(&pos)
            .unwrap();

        chunk.gfx = gfx;
    }

    /// reports all loaded chunks
    pub fn chunks(&self) -> Values<'_, int3, Box<Chunk>>
    {
        self.chunks.values()
    }

    pub fn seed(&self) -> u32
    {
        self.seed
    }
}

impl Index<int3> for Dimension
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
            &chunk[(rx as u32, ry as u32, rz as u32)]
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

impl IndexMut<int3> for Dimension
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
            self.load_chunk(pos);
        }

        self.chunks
            .get_mut(&pos)
            .unwrap()
            .index_mut((rx as u32, ry as u32, rz as u32))
    }
}

impl Index<(i32, i32, i32)> for Dimension
{
    type Output = Block;

    fn index(&self, index: (i32, i32, i32)) -> &Self::Output
    {
        &self[int3::new(index.0, index.1, index.2)]
    }
}

impl IndexMut<(i32, i32, i32)> for Dimension
{
    fn index_mut(&mut self, index: (i32, i32, i32)) -> &mut Self::Output
    {
        &mut self[int3::new(index.0, index.1, index.2)]
    }
}