# `ezecs` branch
```rust
use ezgame::*;

struct SPhysics;

impl System for SPhysics
{
    const EVENT = time::UPDATE;
    const ORDER = Priority::HIGH;

    fn get_fn() -> SysFn
    {
        // begin...
        sys("physics_system")
        // components...
        .with_query(<(Read<Velocity>, Write<Position>)>::query())
        // resources...
        .read_resource::<Time>()
        .read_resource::<Events>()
        // system...
        .build(|_, world, (time, events), query|
        {
            for (vel, mut pos) in query.iter_mut(world)
            {
                pos += vel * time.dt();

                if pos.x > 20
                {
                    events.push("physics_wall_event");
                }
            }
        })
    }
}
```