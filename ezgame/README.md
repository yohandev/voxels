# ezgame
```rust
use ezgame::*;

fn main()
{ 
    Application::run::<TestGame>();
}

struct TestGame;

impl Game for TestGame
{
    fn build(app: &mut Application) -> Self
    {
        // you can have as many worlds as you want.
        // ezgame is powered by Legion, so entities
        // are valid across worlds.
        let world = app.create_world();

        // everything is an entity, including windows!
        let window_components =
        {
            use components::*;

            vec!
            [(
                Window::request(),      // required
                WindowSize::default(),  // optional
                WindowTitle::default()  // optional
            )]
        };

        // finally, "insert the window" into the ECS world
        world.insert((), window_components);

        Self
    }
}
```