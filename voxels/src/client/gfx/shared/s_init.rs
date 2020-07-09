use ezgame::ecs::*;
use ezgame::gfx::*;
use ezgame::*;

use super::{ ViewProjUniform, RGraphicsShared };

/// system that initializes the RGraphicsShared
/// resource
pub struct SGraphicsShared;

impl System for SGraphicsShared
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<ezgame::evt::Start>(-999, Self::on_start);
        handlers.insert::<gfx::evt::Ready>(-999, Self::on_ezgfx_ready);
    }
}

impl SGraphicsShared
{
    fn on_start(app: &mut Application)
    {
        app.resources().insert(RGraphicsShared::None);
    }

    fn on_ezgfx_ready(app: &mut Application)
    {
        if let Some(ctx) = &*app.gfx()
        {
            // uniform buffer
            let vp = ctx.uniform(ViewProjUniform::default());
            // bind group
            let vp = ctx.bind_group(ShaderKind::Vertex, (vp,));

            // explicit typing to make sure type matches
            let res: RGraphicsShared = Some((vp,));

            // replace resource
            app.resources().insert(res);
        }
    }
}