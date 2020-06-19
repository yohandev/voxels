# `custom-ecs` branch proposal
```rust
// -- components --
#[derive(Debug, Default, Clone)]
struct CPosition(f32, f32);
#[derive(Debug, Default, Clone)]
struct CVelocity(f32, f32);

// -- resources --
#[derive(Debug, Default)]
struct RGravity(f32);

// -- events --
#[derive(Debug, Default, Copy, Clone)]
struct StartEvent;
#[derive(Debug, Default, Copy, Clone)]
struct UpdateEvent(f32);

// -- system --
struct SPhysics;

impl System<UpdateEvent> for SPhysics
{
    const PRIORITY = 0;

    type Data =
    (
        CWrite<CPosition>,
        CRead<CVelocity>,

        RRead<RGravity>,
    );

    fn on_event(&mut self, data: (mut pos, vel, gravity))
    {
        // merge pos and vel storage
        // multi-threaded by default
        pos.and(vel).for_each
        (
            |mut pos, vel|
            {
                pos.0 += vel.0;
                pos.1 += vel.1;
            }
        );

        // alternatively, run single-threaded
        for (mut pos, vel) in pos.and(vel).iter()
        {
            pos.0 += vel.0;
            pos.1 += vel.1;
        }
    }
}

impl System<StartEvent> for SPhysics
{
    const PRIORITY = 999;

    type Data =
    (
        RWrite<Gravity>
    );

    fn on_event(&mut self, (mut gravity))
    {
        *gravity = -9.8;
    }
}
```