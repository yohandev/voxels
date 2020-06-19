mod components;
mod resources;
mod systems;
mod game;

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

        // request window
        app.resources().insert
        (
            plugins::winit::resources::WindowRequest::new()
                .width(600)
                .height(600)
                .title("voxels")
        );
        
        // you can have as many worlds as you want.
        // ezgame is powered by Legion, so entities
        // are valid across worlds.
        let world = app.create_world();

        // insert standard camera into world
        let camera_components = 
        {
            use crate::components::transform::*;
            use crate::components::gfx::Camera;

            vec!
            [(
                Camera::new(45f32.to_radians(), 0.01, 1000.0, 1.0, 1.0),
                LocalToWorld::default(),

                Translation(ezmath::float3::new(0.0, 0.0, 10.0)),
                Rotation::default()
            )]
        };
        world.insert((), camera_components);

        // insert chunks into world
        let chunk_components: Vec<(crate::components::game::Chunk,)> =
        {
            use crate::components::game::*;

            (0..5)
                .flat_map(|x| (0..5).map(move |z| (x, z)))
                .map(|(x, z)| (Chunk::new(ezmath::int3::new(x * game::CHUNK_SIZE as i32, 0, z * game::CHUNK_SIZE as i32)),))
                .collect()
        };
        world.insert((crate::components::game::ChunkLoadTag,), chunk_components);

        Self
    }
}