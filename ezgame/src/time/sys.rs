use crate::ecs::*;
use crate::evt;

pub struct STime;

impl System for STime
{
    const EVENT: Event = evt::POLL;
    const ORDER: Order = ord::HIGH;

    fn exe() -> SysFn
    {
        sys("ezgame_time_system");

        todo!()
        //.read_resource()
    }
}