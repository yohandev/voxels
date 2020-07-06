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

    fn run(&mut self, app: &mut crate::Application, evt: &PollEvent)
    {
        if let winit::event::Event::NewEvents(_) = &evt.0
        {
            let now = Instant::now();

            let mut r_time = app
                .resources()
                .get_mut_or_insert_with(|| RTime::new())
                .unwrap();

            r_time.delta = now.duration_since(r_time.frame);
            r_time.frame = now;
        }
        if let winit::event::Event::MainEventsCleared = &evt.0
        {
            let dt = app
                .res()
                .get::<RTime>()
                .unwrap()
                .dt();

            app.invoke(evt::UpdateEvent { dt });
        }
        if let winit::event::Event::RedrawRequested(_) = &evt.0
        {
            app.invoke(evt::RenderEvent);
        }
    }
}