use winit::event_loop::ControlFlow;

use crate::resources::EventsQueue;
use crate::Application;

/// special system that's called by the Application itself to
/// flush and invoke events inside the EventsQueue resource.
pub(crate) fn system_process_invokes(app: &mut Application, flow: &mut ControlFlow)
{
    // flush events
    let events = app
        .resources()
        .remove::<EventsQueue>()
        .unwrap()
        .unwrap();

    // prepare for new events
    app
        .resources()
        .insert(EventsQueue::new());

    // break recursion
    if events.is_empty()
    {
        return;
    }

    // invoke events, in order
    for event in events
    {
        app.invoke(event);

        // special event: quit
        if event == crate::events::APP_QUIT
        {
            *flow = ControlFlow::Exit;
        }
    }

    // recurse the events added by invoking this event
    system_process_invokes(app, flow);
}