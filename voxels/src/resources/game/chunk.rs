use std::collections::hash_map::*;

use ezgame::legion::*;
use ezmath::*;

use crate::components::game::*;
use crate::game::*;

/// stores all the loaded chunks in a int3 <-> entity hashmap
///
/// eventually (maybe?) should be replaced by a ChunkNeighbor
/// coomponent.
#[derive(Debug, Default)]
pub struct LoadedChunks
{
    /// all currently loaded chunks
    pub chunks: HashMap<int3, Entity>,
}

impl LoadedChunks
{
    /// get the block at the world block position
    /// returns Block::default() if block is invalid or
    /// unloaded.
    pub fn get_block(&self, pos: int3, world: &SubWorld) -> Block
    {
        let rx = pos.x.rem_euclid(CHUNK_SIZE as i32);
        let ry = pos.y.rem_euclid(CHUNK_SIZE as i32);
        let rz = pos.z.rem_euclid(CHUNK_SIZE as i32);

        let x = pos.x - rx;
        let y = pos.y - ry;
        let z = pos.z - rz;

        if let Some(ent) = self.chunks.get(&int3::new(x, y, z))
        {
            if let Some(chunk) = &world.get_component::<Chunk>(*ent)
            {
                chunk[(rx as u32, ry as u32, rz as u32)]
            }
            else
            {
                println!("[unexpected] loaded chunks contained stale entity!");

                Block::default()
            }
        }
        else
        {
            // not loaded
           Block::default()
        }
    }

    // pub fn get_block_mut<'a>(&'a mut self, pos: int3, world: &'a SubWorld) -> &mut Block
    // {
    //     let rx = pos.x.rem_euclid(CHUNK_SIZE as i32);
    //     let ry = pos.y.rem_euclid(CHUNK_SIZE as i32);
    //     let rz = pos.z.rem_euclid(CHUNK_SIZE as i32);

    //     let x = pos.x - rx;
    //     let y = pos.y - ry;
    //     let z = pos.z - rz;

    //     let pos = int3::new(x, y, z);

    //     if !self.chunks.contains_key(&pos)
    //     {
    //         // load chunk
    //         //self.load_chunk(pos);
    //     }

    //     self.chunks
    //         .get_mut(&pos)
    //         .unwrap()
    //         .index_mut((rx as u32, ry as u32, rz as u32))
    // }
}