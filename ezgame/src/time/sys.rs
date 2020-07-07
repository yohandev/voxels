use std::time::*;

use crate::ecs::*;
use crate::*;
use super::*;

/// system that manages time for your game,
/// invoking the update and render events
/// and caching time information in the
/// RTime resource.
pub struct STime;

impl System for STime
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<crate::evt::Start>(-9999, Self::on_start);
        handlers.insert::<crate::evt::Poll>(-9999, Self::on_poll);
    }
}

impl STime
{
    fn on_start(app: &mut Application)
    {
        app.resources().insert(RTime::new());
    }

    fn on_poll(app: &mut Application)
    {
        // get resources
        let (r_poll, mut r_time) = app.fetch_mut::<(Read<crate::RWinitPoll>, Write<RTime>)>();

        // new frame
        if let winit::event::Event::NewEvents(_) = &*r_poll
        {
            let now = Instant::now();

            r_time.delta = now.duration_since(r_time.frame);
            r_time.frame = now;
        }
        // update loop
        if let winit::event::Event::MainEventsCleared = &*r_poll
        {
            app.invoke::<super::evt::Update>();
        }
        // render loop
        if let winit::event::Event::RedrawRequested(_) = &*r_poll
        {
            app.invoke::<super::evt::Render>();
        }
    }
}