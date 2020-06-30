use ezgame::time::evt;
use ezgame::ecs::*;
use ezmath::*;

use super::*;

/// system that updates local to world components
pub struct SLocalToWorld;

impl System for SLocalToWorld
{
    const EVENT: Event = evt::UPDATE;
    const ORDER: Order = ord::HIGH * 2;

    fn exe() -> SysFn
    {
        // begin...
        sys("local_to_world_system")
        // components... translation only
        .with_query
        (
            <(Write<CLocalToWorld>, Read<CTranslation>)>::query()
                .filter
                (
                    changed::<CTranslation>() &
                    !component::<CRotation>()
                )
        )
        // components... rotation only
        .with_query
        (
            <(Write<CLocalToWorld>, Read<CRotation>)>::query()
                .filter
                (
                    changed::<CRotation>() &
                    !component::<CTranslation>()
                )
        )
        // components... translation + rotation
        .with_query
        (
            <(Write<CLocalToWorld>, Read<CTranslation>, Read<CRotation>)>::query()
                .filter
                (
                    changed::<CTranslation>() |
                    changed::<CRotation>()
                )
        )
        // system
        .build(|_, world, _, (q_t, q_r, q_tr)|
        {
            for (mut ltw, t) in q_t.iter_mut(world)
            {
                *ltw = CLocalToWorld(float4x4::translation(t.0));
            }

            for (mut ltw, r) in q_r.iter_mut(world)
            {
                *ltw = CLocalToWorld(float4x4::rotation(r.0));
            }

            for (mut ltw, t, r) in q_tr.iter_mut(world)
            {
                *ltw = CLocalToWorld(float4x4::translation(t.0) * float4x4::rotation(r.0))
            }
        })
    }
}