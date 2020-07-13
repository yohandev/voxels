use ezgame::time::evt;
use ezgame::input::*;
use ezgame::ecs::*;
use ezgame::*;
use ezmath::*;

use crate::common::states::GameState;
use crate::common::transform::*;
use crate::client::camera::*;

/// simple first person controller system
pub struct SFpsController;

impl System for SFpsController
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<evt::Update>(0, Self::on_update)
    }
}

impl SFpsController
{
    fn on_update(app: &mut Application)
    {
        // camera transform query
        let q_trans =
        <(
            Write<CTranslation>,
            Write<CRotation>,
            Read<CLocalToWorld>
        )>
        ::query().filter(tag::<TMainCamera>());

        // delta time
        let dt = app.time().dt();

        // input
        let r_in = app.input();

        if let Some(state) = app.state().get_mut::<GameState>()
        {
            for (mut c_pos, mut c_rot, c_ltw) in q_trans.iter_mut(&mut state.registry)
            {
                // speed
                let s = dt * 4.0;

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
        }
    }
}