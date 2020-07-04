use ezgame::input::*;
use ezgame::time::*;
use ezgame::ecs::*;
use ezmath::*;

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
            <(Write<CTranslation>, Write<CRotation>, Read<CLocalToWorld>)>::query()
                .filter(tag::<TMainCamera>())
        )
        // resources...
        .read_resource::<RInput>()
        .read_resource::<RTime>()
        // system...
        .build(|_, world, (r_in, r_time), q_trans|
        {
            for (mut c_pos, mut c_rot, c_ltw) in q_trans.iter_mut(world)
            {
                // speed
                let s = r_time.dt() * 4.0;

                // look around
                if r_in.button_down(MouseButton::Left)
                {
                    c_rot.0.x -= r_in.dy() as f32 * 0.01;
                    c_rot.0.y -= r_in.dx() as f32 * 0.01;
                }

                // horizontal, vertical, up axes
                let mut x = r_in.axis(KeyCode::I, KeyCode::K) * c_ltw.right();
                let mut z = r_in.axis(KeyCode::J, KeyCode::L) * c_ltw.forward();
                let y = r_in.axis(KeyCode::M, KeyCode::Space) * float3::y() * s;

                x.y = 0.0; z.y = 0.0;

                let xmag = x.magnitude();
                let zmag = z.magnitude();
                if xmag > 0.0 { x = (x / xmag) * s; }
                if zmag > 0.0 { z = (z / zmag) * s; }

                // move
                c_pos.0 += (x + y + z) * if r_in.key_down(KeyCode::N) { 3.5 } else { 1.0 };
            }
        })
    }
}