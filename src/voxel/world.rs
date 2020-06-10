use std::collections::HashMap;
use std::ops::*;

use super::*;

pub struct World
{
    /// all currently loaded chunks
    chunks: HashMap<int3, Box<Chunk>>,

    /// dummy air block to return when falsy indexing
    air: Block,
}

impl World
{

}

impl Index<int3> for World
{
    type Output = Block;

    fn index(&self, index: int3) -> &Self::Output
    {
        let rx = index.x % CHUNK_SIZE as i32;
        let ry = index.y % CHUNK_SIZE as i32;
        let rz = index.z % CHUNK_SIZE as i32;

        let x = index.x - rx;
        let y = index.y - ry;
        let z = index.z - rz;

        if let Some(chunk) = self.chunks.get(&int3::new(x, y, z))
        {
            &chunk[(rx, ry, rz)]
        }
        else
        {
            println!
            (
                "[warn] {}<{}, {}, {}>. {}. {}.",
                "attempting to index unloaded chunk! @ ",
                index.x, index.y, index.z,
                "returned air block instead",
                "use mutable indexer to load chunk if needed"
            );

            &self.air
        }
    }
}

impl IndexMut<int3> for World
{
    fn index_mut(&mut self, index: int3) -> &mut Self::Output
    {
        let rx = index.x % CHUNK_SIZE as i32;
        let ry = index.y % CHUNK_SIZE as i32;
        let rz = index.z % CHUNK_SIZE as i32;

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
            self.chunks.insert(pos, Box::new(Chunk::load(&self, pos)));
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