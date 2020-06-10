use noise::*;
use rand::prelude::*;

use super::*;

pub fn generate(world: &World, chunk: &mut Chunk, _: &ThreadRng)
{
    const SEA_LEVEL: f64 = 10.0;
    const TERRAIN_DELTA: f64 = 10.0;

    let perlin = Perlin::new().set_seed(world.seed());

    for rx in 0..CHUNK_SIZE as i32
    {
        for rz in 0..CHUNK_SIZE as i32
        {
            let x = (rx + chunk.x()) as f64;
            let z = (rz + chunk.z()) as f64;

            let h = (perlin.get([x / 15.0, z / 15.0]) * TERRAIN_DELTA + SEA_LEVEL) as i32;
            let rh = h - chunk.y();

            for ry in 0..rh.min(CHUNK_SIZE as i32)
            {
                chunk[(rx, ry, rz)] = Block::new(0b0000_0000_0000_1000);
            }
        }
    }
}

// pub fn generate(world: &World, chunk: &mut Chunk, _: &ThreadRng)
// {
//     for y in 0..CHUNK_SIZE as i32
//     {
//         for z in 0..CHUNK_SIZE as i32
//         {
//             for x in 0..CHUNK_SIZE as i32
//             {
//                 chunk[(x, y, z)] = Block::new(0b0000_0000_0000_1000);
//             }
//         }
//     }
// }