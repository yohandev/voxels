use std::time::*;

use crate::ecs::*;
use crate::evt::*;
use super::*;

/// system that manages time for your game,
/// invoking the update and render events
/// and caching time information in the
/// RTime resource.
pub struct STime;

impl System<PollEvent> for STime
{
    const ORDER: isize = 9999;

    fn run(app: &mut crate::Application)
    {
        let (r_poll, mut r_time) = <(Read<crate::RWinitPoll>, Write<RTime>)>::fetch_mut(app.res_mut());
        // let r_poll = app
        //     .res()
        //     .get::<crate::RWinitPoll>()
        //     .unwrap()
        //     .to_owned();

        if let winit::event::Event::NewEvents(_) = &*r_poll
        {
            let now = Instant::now();

            // let mut r_time = app
            //     .resources()
            //     .get_mut_or_insert_with(|| RTime::new())
            //     .unwrap();

            r_time.delta = now.duration_since(r_time.frame);
            r_time.frame = now;
        }
        if let winit::event::Event::MainEventsCleared = &*r_poll
        {
            app.invoke::<evt::UpdateEvent>();
        }
        if let winit::event::Event::RedrawRequested(_) = &*r_poll
        {
            app.invoke::<evt::RenderEvent>();
        }
    }
}