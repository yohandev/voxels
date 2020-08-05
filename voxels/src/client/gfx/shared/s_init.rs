use ezgame::ecs::*;
use ezgame::gfx::*;

use super::{ ViewProjUniform, RGraphicsShared };

/// system that initializes the RGraphicsShared
/// resource
pub struct SGraphicsShared;

impl System for SGraphicsShared
{
    const EVENT: Event = evt::READY;
    const ORDER: Order = ord::HIGH;

    fn prepare(r: &mut Resources)
    {
        r.insert(RGraphicsShared::None);
    }

    fn exe() -> Job
    {
        // begin...
        sys("shared_graphics_init_system")
        // resources
        .read_resource::<RGraphics>()
        .write_resource::<RGraphicsShared>()
        // system
        .build(move |_, _, (r_gfx, r_shared), _|
        {
            let ctx = r_gfx.as_ref().unwrap();
            
            let vp = ctx.uniform(ViewProjUniform::default());
            let vp = ctx.bind_group(ShaderKind::Vertex, (vp,));

            r_shared.replace((vp,));
        })
    }
}