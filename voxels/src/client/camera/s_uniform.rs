use ezgame::ecs::*;
use ezgame::gfx::*;
use ezgame::time;
use ezgame::*;
use ezmath::*;

use crate::common::transform::CLocalToWorld;
use crate::client::gfx::
{
    RGraphicsShared, 
    ViewProjUniform
};

/// updates the camera view-proj uniform
pub struct SCameraUniform;

impl System for SCameraUniform
{
    fn register(handlers: &mut Systems) 
    {
        handlers.insert::<time::evt::Render>(-499, Self::on_render);
    }
}

impl SCameraUniform
{
    fn on_render(app: &mut Application)
    {
        // camera query
        let q_cam = <(Read<super::CCamera>, TryRead<CLocalToWorld>)>::query()
            .filter
            (
                tag::<super::TMainCamera>() &
                (changed::<super::CCamera>() |
                changed::<CLocalToWorld>())
            );
           
        // fetch resources
        let (mut r_gfx, r_shared) = app.fetch_mut::<(Write<RGraphics>, Read<RGraphicsShared>)>();

        // do nothing is renderer or resources
        // aren't initialized
        if r_gfx.is_none()
        {
            return;
        }
        if r_shared.is_none()
        {
            return;
        }
        let ctx = r_gfx.as_mut().unwrap();
        let shared = r_shared.as_ref().unwrap();

        // get first camera
        for (cam, ltw) in q_cam.iter(app.registry())
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
            ctx.update_uniform(&shared.0.bindings.0, ViewProjUniform::new(vp));

            // break after first camera
            return;
        }
    }
}