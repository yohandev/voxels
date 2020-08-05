use ezgame::ecs::*;
use ezgame::*;
use ezmath::*;

use crate::common::CHUNK_SIZE;
use super::*;

/// chunk loading system
pub struct SChunkLoad;

impl System for SChunkLoad
{
    const EVENT: Event = evt::START;    // temporary
    const ORDER: Order = ord::MID;
    
    fn prepare(res: &mut Resources)
    {
        res.insert(RChunkCache::new())
    }

    fn exe() -> Job
    {
        // begin...
        sys("chunk_loading_system")
        // resources...
        .write_resource::<RChunkCache>()
        // system...
        .build(|cmd, _, r_cache, _|
        {
            // temporary load on start
            (0..5)
                .flat_map(|x| (0..5).map(move |z| (x, z)))
                .flat_map(|(x, z)| (-2..2).map(move |y| (x, y, z)))
                .for_each(|(x, y, z)|
                {
                    r_cache.load(cmd, int3::new(x, y, z) * CHUNK_SIZE as i32);
                });
        })
    }
}