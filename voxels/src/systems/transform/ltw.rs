use ezgame::legion::*;
use ezmath::*;

use crate::components::transform::*;

pub fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("local_to_world_system")
        // components: translation only
        .with_query
        (
            <(Write<LocalToWorld>, Read<Translation>)>::query()
                .filter
                (
                    changed::<Translation>() &
                    !component::<Rotation>()
                )
        )
        // components: rotation only
        .with_query
        (
            <(Write<LocalToWorld>, Read<Rotation>)>::query()
                .filter
                (
                    changed::<Rotation>() &
                    !component::<Translation>()
                )
        )
        // components translation + rotation
        .with_query
        (
            <(Write<LocalToWorld>, Read<Translation>, Read<Rotation>)>::query()
                .filter
                (
                    changed::<Translation>() |
                    changed::<Rotation>()
                )
        )
        .build(|_, world, _, (t, r, tr)|
        {
            for (mut ltw, t) in t.iter_mut(world)
            {
                *ltw = LocalToWorld(float4x4::translation(t.0));
            }

            for (mut ltw, r) in r.iter_mut(world)
            {
                *ltw = LocalToWorld(float4x4::rotation(r.0));
            }

            for (mut ltw, t, r) in tr.iter_mut(world)
            {
                *ltw = LocalToWorld(float4x4::translation(t.0) * float4x4::rotation(r.0))
            }
        })
}