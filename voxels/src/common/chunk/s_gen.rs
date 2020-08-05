use ezgame::time::evt;
use ezgame::ecs::*;

use super::{ CChunk, CBlockBuffer, BlockBufferIndex, TUngenerated, TUpdated };
use crate::common::block::PackedBlock;
use crate::common::CHUNK_SIZE;

/// system that generates chunks'
/// terrain
pub struct SChunkGen;

impl System for SChunkGen
{
    const EVENT: Event = evt::UPDATE;
    const ORDER: Order = ord::LOW;

    const FLUSH: bool = true;

    fn exe() -> Job
    {
        // begin...
        sys("chunk_gen_system")
        // components...
        .with_query
        (
            <(Read<CChunk>, Write<CBlockBuffer>)>::query()
                .filter(tag::<TUngenerated>())
        )
        .build(|cmd, world, _, q_chunks|
        {
            /// sea level at which terrain is generated
            const SEA_LEVEL: f64 = 10.0;
            /// up and down delta from sea level at which terrain is generated
            const TERRAIN_DELTA: f64 = 5.0;

            use noise::*;

            let perlin = Perlin::new().set_seed(12345);

            for (ent, (chunk, mut blocks)) in q_chunks.iter_entities_mut(world)
            {
                // go through horizontal plane
                for rx in 0..CHUNK_SIZE as u32
                {
                    for rz in 0..CHUNK_SIZE as u32
                    {
                        // global block
                        let x = (rx as i32 + chunk.pos().x) as f64;
                        let z = (rz as i32 + chunk.pos().z) as f64;

                        // global height
                        let h = (perlin.get([x / 15.0, z / 15.0]) * TERRAIN_DELTA + SEA_LEVEL) as i32;
                        
                        // relative height
                        let rh = h - chunk.pos().y;

                        // fill all 0..32 or none 0..-n blocks
                        for ry in 0..rh.min(CHUNK_SIZE as i32)
                        {   
                            blocks.set_packed((rx, ry, rz),
                            {
                                if ry == rh - 1
                                {
                                    PackedBlock::new(0b0000_0000_0001_0000)
                                }
                                else
                                {
                                    PackedBlock::new(0b0000_0000_0010_0000)
                                }
                            });
                        }
                    }
                }

                println!("generated chunk!");

                // remove and set tags
                cmd.remove_tag::<TUngenerated>(ent);
                cmd.add_tag(ent, TUpdated);
            }
        })
    }
}