# `ecs-no-closure` branch
## because systems don't need to run in parallel

benchmark of average event processing:
- parallel(`add_system()`): 2.9808742ms
    - let legion optimize parallelism
- local(`flush()`): 2.789957ms
    - flush everytime(rough test)

## proposal
```rust
struct SPhysics;

impl System<StartEvent> for SPhysics
{
    const ORDER: isize = 0;

    fn run(app: &mut Application)
    {
        app.resources().insert(RGravity { x: 0.0, y: -9.8, z: 0.0 });
    }
}

impl System<StateChangeEvent> for SPhysics
{
    const ORDER: isize = 999;

    fn run(&mut self, app: &mut Application, evt: &StateChangeEvent)
    {
        if app.state().is::<GameplayState>() && !self.added
        {
            app.registry().insert((), vec![(CVel::new(), CPos::new())])

            self.added = true;
        }
    }
}

impl System<UpdateEvent> for SPhysics
{
    const ORDER: isize = 0;

    fn run(&mut self, app: &mut Application, evt: &UpdateEvent)
    {
        if app.state().is::<PauseState>()
        {
            return
        }
        let registry = app.registry(); // short-cut to app.state().registry()

        let gravity = app.resources().get::<RGravity>();
        let time = app.time(); // short-cut to app.resources().get::<RTime>()

        // acceleration -> velocity
        for mut vel in <Write<CVel>>::query().iter_mut(registry)
        {
            vel -= gravity;
        }

        // velocity -> position
        for (mut pos, vel) in <(Write<CPos>, Read<CVel>)>::query().iter_mut(registry)
        {
            pos += vel;
        }
    }
}

struct PauseState;
struct GameplayState;

struct TestGame;

impl Game for TestGame
{
    fn build(app: &mut Application)
    {
        app.systems().insert::<SPhysics>();
        // or
        app.systems().insert_val(SPhysics { added: true })

        app.systems().bundle::<GameBundle>()
            .build();
        app.systems().bundle::<RenderBundle>() // returns RenderBundle::Builder
            .width(600)
            .height(400)
            .title("my window")
            .clear(0.1, 0.2, 0.3, 1.0)
            .build();
        
        app.states().insert::<PauseState>();
        app.states().insert::<GameplayState>();

        app.states().switch::<GameplayState>(); // inserts if not inserted
    }
}
```