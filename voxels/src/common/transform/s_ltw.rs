use ezgame::time::evt;
use ezgame::ecs::*;
use ezgame::*;
use ezmath::*;

use super::*;

/// system that updates local to world components
pub struct SLocalToWorld;

impl System for SLocalToWorld
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<evt::Update>(-999, Self::on_update);
    }
}

impl SLocalToWorld
{
    fn on_update(app: &mut Application)
    {
        // translation only
        let q_t = <(Write<CLocalToWorld>, Read<CTranslation>)>::query()
            .filter
            (
                changed::<CTranslation>() &
                !component::<CRotation>()
            );
        // rotation only
        let q_r = <(Write<CLocalToWorld>, Read<CRotation>)>::query()
            .filter
            (
                changed::<CRotation>() &
                !component::<CTranslation>()
            );
        // translation + rotation
        let q_tr = <(Write<CLocalToWorld>, Read<CTranslation>, Read<CRotation>)>::query()
            .filter
            (
                changed::<CTranslation>() |
                changed::<CRotation>()
            );

        for &mut registry in app.registries_mut()
        {
            for (mut ltw, t) in q_t.iter_mut(&mut registry)
            {
                *ltw = CLocalToWorld(float4x4::translation(t.0));
            }

            for (mut ltw, r) in q_r.iter_mut(&mut registry)
            {
                *ltw = CLocalToWorld(float4x4::rotation(r.0));
            }

            for (mut ltw, t, r) in q_tr.iter_mut(&mut registry)
            {
                *ltw = CLocalToWorld(float4x4::translation(t.0) * float4x4::rotation(r.0))
            }
        }
    }
}