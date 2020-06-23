# `systems` branch
refactors the way systems works

```rust
trait System
{
    fn build(Application) -> Schedulable;
}

trait SystemBundle
{
    fn build(Application);
}

impl Game for game!()
{
    fn build(app: &mut Application)
    {
        {
            app.systems().insert(ezgame::InputBundle);
            app.system().insert(ezgame::WindowBundle);
            app.system().insert(ezgame::TimeBundle);
        }
        // OR
        {
            app.systems().insert(ezgame::GameBundle);
        }

        // any 'stray' systems inserted 
        // before is merged into a par
        // schedule
        app.systems().flush();

        app.systems().insert(MySystemA);
        app.systems().insert(MySystemB);

        // A and B run in parallel
        app.systems().flush();

        // C runs after A & B
        app.systems().insert(MySystemC);
    }
}
```