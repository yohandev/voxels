use std::time::*;

use crate::ecs::*;
use crate::evt;

/// system that manages time for your game,
/// invoking the update and render events
/// and caching time information in the
/// RTime resource.
pub struct STime;

impl System for STime
{
    const EVENT: Event = evt::POLL;
    const ORDER: Order = ord::HIGH;

    fn prepare(r: &mut Resources)
    {
        r.insert(super::RTime::new());
    }

    fn exe() -> SysFn
    {
        // begin...
        sys("ezgame_time_system")
        // resources...
        .read_resource::<crate::window::REvent>()
        .write_resource::<super::RTime>()
        .read_resource::<REvents>()
        // system...
        .build(|_, _, (r_winit, r_time, r_events), _|
        {
            if let winit::event::Event::NewEvents(_) = &**r_winit
            {
                let now = Instant::now();

                r_time.delta = now.duration_since(r_time.frame);
                r_time.frame = now;
            }
            if let winit::event::Event::MainEventsCleared = &**r_winit
            {
                r_events.push(evt::UPDATE);
            }
            if let winit::event::Event::RedrawRequested(_) = &**r_winit
            {
                r_events.push(evt::RENDER);
            }
        })
    }
}