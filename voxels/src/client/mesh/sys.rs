use ezgame::ecs::*;
use ezgame::gfx::*;

/// system that meshes chunks
pub struct SChunkMesher;

impl System for SChunkMesher
{
    const EVENT: Event = evt::READY;
    const ORDER: Order = ord::HIGH;

    const FLUSH: bool = true;

    fn exe() -> SysFn
    {
        todo!()
    }
}