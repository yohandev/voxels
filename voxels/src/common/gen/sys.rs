use ezgame::time::evt;
use ezgame::ecs::*;
use noise::*;

use crate::common::world::*;

/// system that generates the terrain
/// of chunks as they're loaded.
pub struct SGenerateChunk;

impl System for SGenerateChunk
{
    const EVENT: Event = evt::UPDATE;
    const ORDER: Order = ord::MID;

    fn exe() -> SysFn
    {
        // begin...
        sys("gen_chunk_system")
        // resources...
        .write_resource::<RWorld>()
        // system...
        .build(|_, _, r_world, _|
        {
            /// sea level at which terrain is generated
            const SEA_LEVEL: f64 = 10.0;
            /// up and down delta from sea level at which terrain is generated
            const TERRAIN_DELTA: f64 = 5.0;

            let perlin = Perlin::new().set_seed(12345);

            for chunk in r_world
                .chunks_mut()
                .filter(|c| c.state() == ChunkState::Loading)
            {
                // go through horizontal plane
                for rx in 0..CHUNK_SIZE as u32
                {
                    for rz in 0..CHUNK_SIZE as u32
                    {
                        // global block
                        let x = (rx as i32 + chunk.id().x) as f64;
                        let z = (rz as i32 + chunk.id().z) as f64;

                        // global height
                        let h = (perlin.get([x / 15.0, z / 15.0]) * TERRAIN_DELTA + SEA_LEVEL) as i32;
                        
                        // relative height
                        let rh = h - chunk.id().y;

                        // fill all 0..32 or none 0..-n blocks
                        for ry in 0..rh.min(CHUNK_SIZE as i32)
                        {
                            chunk[(rx, ry as u32, rz)] = Block::new(0b0000_0000_0000_1000);
                        }
                    }
                }
            }
        })
    }
}