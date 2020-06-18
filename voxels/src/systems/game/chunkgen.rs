use ezgame::legion::*;

use crate::components::game::*;
use crate::game::*;

/// sea level at which terrain is generated
pub const SEA_LEVEL: f64 = 10.0;
/// up and down delta from sea level at which terrain is generated
pub const TERRAIN_DELTA: f64 = 5.0;

/// system that generates chunks' blocks who have both the Chunk
/// component and the ChunkGenerateTag tag
pub fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("chunk_generation_system")
        // components
        .with_query(<Write<Chunk>>::query().filter(tag::<ChunkGenerateTag>()))
        .write_component::<ChunkGenerateTag>()
        // systems
        .build(|cmd, world, _, query|
        {
            use noise::*;

            let perlin = Perlin::new().set_seed(12345);

            for (ent, mut chunk) in query.iter_entities_mut(world)
            {
                // go through horizontal plane
                for rx in 0..CHUNK_SIZE as u32
                {
                    for rz in 0..CHUNK_SIZE as u32
                    {
                        // global block
                        let x = (rx as i32 + chunk.position().x) as f64;
                        let z = (rz as i32 + chunk.position().z) as f64;

                        // global height
                        let h = (perlin.get([x / 15.0, z / 15.0]) * TERRAIN_DELTA + SEA_LEVEL) as i32;
                        
                        // relative height
                        let rh = h - chunk.position().y;

                        // fill all 0..32 or none 0..-n blocks
                        for ry in 0..rh.min(CHUNK_SIZE as i32)
                        {
                            chunk[(rx, ry as u32, rz)] = Block::new(0b0000_0000_0000_1000);
                        }
                    }
                }

                // remove and set tags
                cmd.remove_tag::<ChunkGenerateTag>(ent);
            }
        })
}