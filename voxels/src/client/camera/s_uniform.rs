use ezgame::ecs::*;
use ezgame::gfx::*;
use ezgame::time;
use ezmath::*;

/// updates the camera view-proj uniform
pub struct SCameraUniform;

impl System for SCameraUniform
{
    const EVENT: Event = time::evt::RENDER;
    const ORDER: Order = ord::HIGH;

    const FLUSH: bool = true;

    fn exe() -> SysFn
    {
        // begin...
        sys("camera_update_uniform_system")
        // components...
        .with_query
        (
            <(
                Read<super::CCamera>,
                TryRead<crate::shared::transform::CLocalToWorld>
            )>::query()
            .filter
            (
                tag::<super::TMainCamera>() &
                (changed::<super::CCamera>() |
                changed::<crate::shared::transform::CLocalToWorld>())
            )
        )
        // resources...
        .read_resource::<super::super::RGraphicsShared>()
        .write_resource::<RGraphics>()
        // system...
        .build(|_, world, (r_shared, r_gfx), q_cam|
        {
            // do nothing is renderer or shared
            // aren't initialized
            if r_gfx.is_none()
            {
                return;
            }
            if r_shared.is_none()
            {
                return;
            }

            // unwrap
            let ctx = r_gfx.as_mut().unwrap();
            let shared = r_shared.as_ref().unwrap();

            // get first camera
            for (cam, ltw) in q_cam.iter(world)
            {
                // (view) projection matrix
                let vp = if let Some(ltw) = &ltw
                {
                    cam.proj * ltw.0.inverse()
                }
                else
                {
                    cam.proj
                };

                // update uniforms
                ctx.update_uniform(&shared.0.bindings.0, super::super::ViewProjUniform::new(vp));

                // break after first camera
                break;
            }
        })
    }
}