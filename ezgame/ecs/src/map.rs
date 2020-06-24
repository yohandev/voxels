use std::collections::HashMap;

use crate::*;

/// maps events to systems schedules.
#[derive(Default)]
pub struct EventSystems
{
    map: HashMap<Event, SystemOven>,
}

/// enum for a `Systems` and its `Builder`,
/// depending  on whether it has been "baked"
/// or not.
enum SystemOven
{
    Baked(Systems),
    Baking(Option<Builder>),
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
                // finalize build
                if let SystemOven::Baking(b) = systems
                {
                    *systems = SystemOven::Baked
                    (
                        b.take()
                        .unwrap()
                        .build()
                    );
                }
                
                // run systems
                if let SystemOven::Baked(sys) = systems
                {
                    sys.execute(registry, resources);
                }
            }
        }
        
        // recurse
        self.process(registry, resources);
    }

    /// insert a system into this systems collection,
    /// automatically figuring out ordering and events.
    pub fn insert<T: System>(&mut self)
    {
        let e = T::EVENT;

        match self.map.get_mut(&e)
        {
            // oven already there
            Some(oven) => match oven
            {
                // err: already baked
                SystemOven::Baked(_) => panic!("attempting to add a system after baking!"),
                // baking, simply append!
                SystemOven::Baking(b) => b.as_mut().unwrap().push::<T>(),
            }
            // create new oven and append
            None =>
            {
                // insert oven
                self.map.insert
                (
                    e, SystemOven::Baking(Builder::default().into())
                );
                // baking, append!
                self.insert::<T>();
            }
        }
    }
}