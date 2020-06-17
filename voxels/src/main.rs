mod components;
mod resources;
mod systems;

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
        // add common systems and resources. they won't
        // interfere with any of your components and may
        // impact some ezgame provided ones, but adding
        // these might prevent some headaches and
        // weird behaviours. 
        app.add_defaults();

        // add rendering plugin's sytems and resources
        app.add_plugin_ezgfx();

        // add your resources here
        resources::add_resources(app);

        // add your systems here
        systems::add_systems(app);

        // you can have as many worlds as you want.
        // ezgame is powered by Legion, so entities
        // are valid across worlds.
        let world = app.create_world();

        // everything is an entity, including windows!
        let window_components =
        {
            use ezgame::plugins::ezgfx::components::*;
            use ezgame::components::*;

            vec!
            [(
                Window::request(),      // required
                WindowSize::default(),  // optional
                WindowTitle::default(), // optional
                Graphics::default(),    // plugin required
            )]
        };

        // finally, "insert the window" into the ECS world
        world.insert((), window_components);

        Self
    }
}