use std::collections::HashMap;

use ezgame::ecs::*;
use ezmath::*;

/// caches chunk using a hashmap
pub struct RChunkCache
{
    map: HashMap<int3, Entity>,
}

impl RChunkCache
{
    /// stores a chunk in the cache
    pub fn store(pos: int3, ent: Entity)
    {

    }

    pub fn release()
    {

    }

    pub fn at()
    {

    }
}