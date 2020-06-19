use crate::resources::{ Time, WinitEvent, EventsQueue };

use std::time::*;

use super::*;

/// system that manages time for your game, invoking the
/// "app_update_event" and caching time information in the
/// ezgame::resources::Time resource.
pub fn time_system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("time_system")
        .read_resource::<WinitEvent>()
        .write_resource::<Time>()
        .write_resource::<EventsQueue>()
        .build(|_, _, (event, time_res, invoke), _|
        {
            if let winit::event::Event::NewEvents(_) = event.0
            {
                let now = Instant::now();

                time_res.delta = now.duration_since(time_res.frame);
                time_res.frame = now;
            }
            if let winit::event::Event::MainEventsCleared = event.0
            {
                invoke.invoke(crate::events::APP_UPDATE);
            }
            if let winit::event::Event::RedrawRequested(_) = event.0
            {
                invoke.invoke(crate::events::APP_RENDER);
            }
        })
}