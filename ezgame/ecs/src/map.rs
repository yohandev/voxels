use std::collections::HashMap;

use crate::*;

/// maps events to systems schedules.
pub struct EventSystems
{
    map: HashMap<Event, Systems>
}

impl EventSystems
{
    /// recursively process all events inside
    /// the REvents resource.
    pub fn process(&mut self, registry: &mut Registry, resources: &mut Resources)
    {
        // retrieve events
        let events = resources
            .remove::<REvents>()
            .unwrap();
        
        // replace events
        resources
            .insert(REvents::default());

        // break recursion
        if events.is_empty()
        {
            return;
        }

        // process all events
        while let Some(e) = events.pop()
        {
            // get systems
            if let Some(systems) = self.map.get_mut(&e)
            {
                // run systems
                systems.execute(registry, resources);
            }
        }
        
        // recurse
        self.process(registry, resources);
    }
}