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
    /// the REvents resource. if some systems
    /// aren't built, they will be built dynamically.
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
            if let Some(oven) = self.map.get_mut(&e)
            {
                // finalize build
                if let SystemOven::Baking(b) = oven
                {
                    *oven = SystemOven::Baked
                    (
                        b
                        .take()
                        .unwrap()
                        .build(resources)
                    );
                }
                
                // run systems
                if let SystemOven::Baked(sys) = oven
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

    /// insert all the systems in a bundle,
    /// automatically figuring out ordering and events.
    pub fn bundle<T: SystemBundle>(&mut self)
    {
        T::insert(self);
    }

    /// explicitely build all systems currently
    /// added, inserting their resources into the
    /// app.
    pub fn build(&mut self, resources: &mut Resources)
    {
        // go through every event arbitrarily
        for oven in self.map.values_mut()
        {
            // only build unbaked
            if let SystemOven::Baking(b) = oven
            {
                // bake
                *oven = SystemOven::Baked
                (
                    b
                    .take()
                    .unwrap()
                    .build(resources)
                );
            }
        }
    }
}