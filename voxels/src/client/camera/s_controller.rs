use ezgame::input::*;
use ezgame::time::*;
use ezgame::ecs::*;

use crate::common::transform::*;
use crate::client::camera::*;

/// simple first person controller system
pub struct SFpsController;

impl System for SFpsController
{
    const EVENT: Event = evt::UPDATE;
    const ORDER: Order = ord::MID;

    fn exe() -> SysFn
    {
        // begin...
        sys("camera_fps_controller_system")
        // components...
        .with_query
        (
            <(Write<CTranslation>, Write<CRotation>)>::query()
                .filter(tag::<TMainCamera>())
        )
        // resources...
        .read_resource::<RInput>()
        .read_resource::<RTime>()
        // system...
        .build(|_, world, (r_in, r_time), q_trans|
        {
            for (mut pos, _) in q_trans.iter_mut(world)
            {
                let s = r_time.dt() * 3.5;

                if r_in.key_down(KeyCode::K)     { pos.0.z += s; } else if r_in.key_down(KeyCode::I) { pos.0.z -= s; }
                if r_in.key_down(KeyCode::L)     { pos.0.x += s; } else if r_in.key_down(KeyCode::J) { pos.0.x -= s; }
                if r_in.key_down(KeyCode::Space) { pos.0.y += s; } else if r_in.key_down(KeyCode::M) { pos.0.y -= s; }
            }
        })
    }
}