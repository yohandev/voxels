use ezgame::plugins::ezgfx::*;
use ezgame::legion::*;

use crate::resources::gfx::*;

pub(super) fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("render_system")
        // components
        .with_query(<Write<components::Graphics>>::query())
        // resources
        .write_resource::<SimpleGfxResources>()
        // system
        .build(move |_, world, res, query|
        {
            //let pipeline = res.pipeline.as_ref().unwrap();            

            for mut ctx in query.iter_mut(world)
            {
                ctx.render_pass
                (
                    |_, mut pass|
                    {
                        
                        pass.begin_clear(0.1, 0.2, 0.3, 1.0);
                        //pass.pipeline(&res.pipeline);
                    }
                );
            }

            //res.pipeline.replace(pipeline);
        })
}