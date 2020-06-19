use ezgame::plugins::winit::resources::*;
use ezgame::legion::*;

use crate::components::gfx::*;

/// updates the camera matrix on window resize
pub fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("camera_resize_system")
        // components
        .with_query(<Write<Camera>>::query())
        // resources
        .read_resource::<Window>()
        // system
        .build(|_, world, window, query|
        {
            let size = window
                .as_ref()
                .unwrap()
                .inner_size();

            for mut cam in query.iter_mut(world)
            {
                cam.resize(size.width as f32, size.height as f32);
            }
        })
}