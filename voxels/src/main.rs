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

        // rendering
        app.add_plugin_ezgfx();

        // add your systems here
        app.register_system(ezgame::events::APP_UPDATE_EVENT, my_system());
        app.register_system(ezgame::events::APP_RENDER_EVENT, my_rendering_system());

        // you can have as many worlds as you want.
        // ezgame is powered by Legion, so entities
        // are valid across worlds.
        let world = app.create_world();

        // everything is an entity, including windows!
        let window_components =
        {
            use plugins::ezgfx::components::*;
            use components::*;

            vec!
            [(
                Window::request(),      // required
                WindowSize::default(),  // optional
                WindowTitle::default(), // optional
                Renderer::default(),    // plugin required
            )]
        };

        // finally, "insert the window" into the ECS world
        world.insert((), window_components);

        Self
    }
}

fn my_system() -> Box<dyn legion::Schedulable>
{
    use resources::{ Time, Input };

    use winit::event::VirtualKeyCode;
    use legion::*;

    SystemBuilder::new("my_system")
        .read_resource::<Time>()
        .read_resource::<Input>()
        .build(|_, _, (time, input), _|
        {
            print!("frame {{ delta_time: {}ms", time.delta_time_f32() * 1000.0);

            if input.key_down(VirtualKeyCode::Space)
            {
                print!(", space is down!");
            }
            else
            {
                print!(", space is up!");
            }

            println!(" }}");
        })
}

fn my_rendering_system() -> Box<dyn legion::Schedulable>
{
    use plugins::ezgfx::components::Renderer;

    use legion::*;

    SystemBuilder::new("my_rendering_sytem")
        .with_query(<Write<Renderer>>::query())
        .build(|_, world, _, query|
        {
            for mut ctx in query.iter_mut(world)
            {
                ctx.render_pass(|_, mut pass|
                {
                    pass.begin_clear(0.1, 0.2, 0.3, 1.0);
                });
            }
        })
}