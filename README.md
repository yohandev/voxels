# `custom-ecs` branch proposal
```rust
// -- components --
#[component]
struct CPosition(f32, f32);
#[component]
struct CVelocity(f32, f32);

// -- resources --
#[resource]
struct RGravity(f32);

// -- events --
#[event]
struct StartEvent;
#[event]
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

fn build(app: &mut Application)
{
    // create a new registry
    app.registry();

    // add the system
    app.system(SPhysics);
}
```