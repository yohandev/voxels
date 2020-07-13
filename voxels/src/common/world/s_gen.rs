use ezgame::time::evt;
use ezgame::ecs::*;
use ezgame::*;

use crate::common::block::PackedBlock;
use crate::common::states::GameState;
use crate::common::CHUNK_SIZE;
use super::ChunkIndex;

/// system that generates chunks'
/// terrain
pub struct SChunkGen;

impl System for SChunkGen
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<evt::Update>(-500, Self::on_update);
    }
}

impl SChunkGen
{
    fn on_update(app: &mut Application)
    {
        /// sea level at which terrain is generated
        const SEA_LEVEL: f64 = 10.0;
        /// up and down delta from sea level at which terrain is generated
        const TERRAIN_DELTA: f64 = 5.0;

        use noise::*;

        let perlin = Perlin::new().set_seed(12345);

        if let Some(state) = app.state().get_mut::<GameState>()
        {
            for chunk in state.world.chunks_mut().filter(|c| !c.generated())
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
                            chunk.set_packed((rx, ry, rz),
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
                chunk.mark_generated();
            }
        }
    }
}