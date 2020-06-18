use ezgame::resources::input::{ Input, KeyCode };
use ezgame::legion::*;

use crate::components::transform::*;
use crate::components::gfx::Camera;

/// first person camera controller
pub fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("camera_controller_system")
        .with_query
        (
            <(Write<Translation>, Write<Rotation>)>::query()
                .filter(component::<Camera>())
        )
        .read_resource::<Input>()
        .build(|_, world, input, query|
        {
            for (mut pos, _) in query.iter_mut(world)
            {
                if input.key_down(KeyCode::K) { pos.0.z += 0.075; } else if input.key_down(KeyCode::I) { pos.0.z -= 0.075; }
                if input.key_down(KeyCode::L) { pos.0.x += 0.075; } else if input.key_down(KeyCode::J) {  pos.0.x -= 0.075; }
                if input.key_down(KeyCode::Space) { pos.0.y += 0.075; } else if input.key_down(KeyCode::M) {  pos.0.y -= 0.075; }
            }
        })
}