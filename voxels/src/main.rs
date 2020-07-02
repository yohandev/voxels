//mod components;
//mod resources;
//mod systems;
mod common;
mod client;

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

        app.systems().bundle::<common::Bundle>();
        app.systems().bundle::<client::Bundle>();

        // request window
        app.resources().insert
        (
            window::RWindowRequest::new()
                .width(600)
                .height(600)
                .title("voxels")
        );
        app.resources().insert(common::block::RBlockPalette::load(""));
        
        // insert standard camera into world
        let camera_components = 
        {
            use crate::common::transform::*;
            use crate::client::camera::*;

            (
                (TMainCamera,),
                vec!
                [(
                    CCamera::new(45f32.to_radians(), 0.01, 1000.0, 1.0, 1.0),
                    CLocalToWorld::default(),

                    CTranslation(ezmath::float3::new(0.0, 0.0, 10.0)),
                    CRotation::default()
                )]
            )
        };
        app
            .registry()
            .insert(camera_components.0, camera_components.1);

        Self
    }
}