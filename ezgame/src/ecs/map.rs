use std::collections::HashMap;
use std::any::*;

use super::*;

/// maps events to systems schedules.
#[derive(Default)]
pub struct EventSystems
{
    /// maps a system type to a system object
    typ_to_sys: HashMap<TypeId, Box<dyn Any>>,
    /// maps an event type to a vector of system types
    evt_to_sys: HashMap<TypeId, Vec<(isize, TypeId)>>,
}

impl EventSystems
{
    /// inserts a system into the app by type, using its
    /// default.
    ///
    /// there can only ever be one type of system in the world,
    /// so adding `SMySystem` twice would silently overwrite it.
    pub fn insert<T: System<K> + Default, K: 'static>(&mut self)
    {
        self.insert_val(T::default())
    }

    /// inserts a system by value, letting the user initialize
    /// the system data
    ///
    /// there can only ever be one type of system in the world,
    /// so adding `SMySystem` twice would silently overwrite it.
    pub fn insert_val<T: System<K>, K: 'static>(&mut self, sys: T)
    {
        let sys_id = TypeId::of::<T>();
        let evt_id = TypeId::of::<K>();

        if let Some(_) = self.typ_to_sys.insert(sys_id, Box::new(sys))
        {
            println!("[warn] inserted a system twice, so the old one was overwritten");
        }
        else
        {
            // first system for event
            if !self.evt_to_sys.contains_key(&evt_id)
            {
                self.evt_to_sys.insert(evt_id, Default::default());
            }

            // get all systems for event
            let map = self.evt_to_sys
                .get_mut(&evt_id)
                .unwrap();
            
            // insert and sort
            map.push((T::ORDER, sys_id));
            map.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        }
    }
}