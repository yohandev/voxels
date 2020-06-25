use ezgame::time::evt;
use ezgame::ecs::*;

/// system that renders 3D models
pub struct SRender;

impl System for SRender
{
    const EVENT: Event = evt::RENDER;
    const ORDER: Order = ord::MID;

    const FLUSH: bool = true;

    fn exe() -> SysFn
    {
        todo!()
    }
}