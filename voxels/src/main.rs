//mod components;
//mod resources;
//mod systems;
mod client;
mod shared;

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
        app.systems().bundle::<GameBundle>();

        app.systems().bundle::<shared::Bundle>();
        app.systems().bundle::<client::Bundle>();

        // request window
        app.resources().insert
        (
            window::RWindowRequest::new()
                .width(600)
                .height(600)
                .title("voxels")
        );
        
        // insert standard camera into world
        //let camera_components = 
        //{
            // use crate::components::transform::*;
            // use crate::components::gfx::Camera;

            // vec!
            // [(
            //     Camera::new(45f32.to_radians(), 0.01, 1000.0, 1.0, 1.0),
            //     LocalToWorld::default(),

            //     Translation(ezmath::float3::new(0.0, 0.0, 10.0)),
            //     Rotation::default()
            // )]
        //};
        //app.registry().insert((), camera_components);

        // insert chunks into world
        // let chunk_components: Vec<(crate::components::game::Chunk,)> =
        // {
        //     use crate::components::game::*;

        //     (0..5)
        //         .flat_map(|x| (0..5).map(move |z| (x, z)))
        //         .map(|(x, z)| (Chunk::new(ezmath::int3::new(x * game::CHUNK_SIZE as i32, 0, z * game::CHUNK_SIZE as i32)),))
        //         .collect()
        // };
        // world.insert((crate::components::game::ChunkLoadTag,), chunk_components);

        Self
    }
}