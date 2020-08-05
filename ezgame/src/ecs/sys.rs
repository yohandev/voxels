use std::collections::*;
use std::any::*;

use crate::Application;
use super::Event;

/// an event-responding system, that operates on entities
/// and their components. it's convention that systems
/// are prefixed with an `S`. all the implementation of
/// the `Systems` trait does is the `register` method, where
/// you bind jobs to events.
///
/// # example
/// ```rust
/// struct SPhysics;
/// 
/// impl System for SPhysics
/// {
///     fn build(app: &mut Application)
///     {
///         app.resources().insert(RGravity(-9.8));
///         
///                                         //  (order, flush?, job)
///         app.systems().insert::<evt::Start>  (+000, false, Self::on_start());
///         app.systems().insert::<evt::Update> (-999, false, Self::on_update());
///         app.systems().insert::<evt::Quit>   (+999, false, Self::on_quit());
///     }
/// }
///
/// impl SPhysics
/// {
///     // system functions must have this signature
///     fn on_start() -> Job
///     {
///         // ...
///     }
///     
///     // you can call these methods whatever, but `on_{event}` is
///     // good practice
///     fn on_update() -> Job
///     {
///         // begin...
///         job("physics_on_update")
///         // resources...
///         .read_resource::<RTime>()
///         .read_resource::<RGravity>()
///         // components...
///         .with_query(<Write<CVel>>::query())
///         .with_query(<(Write<CPos>, Read<CVel>)>::query())
///         // system...
///         .build(|_, scene, (time, gravity), (q_vel, q_pos_vel)|
///         {
///             for mut vel in q_vel.iter_mut(&mut scene)
///             {
///                 vel += gravity.0 * time.dt();
///             }
///             
///             for (mut pos, vel) in q_pos_vel.iter_mut(&mut scene)
///             {
///                 pos += vel * time.dt();
///             }
///         })
///     }
///     
///     // event handlers also don't need to be inside an `impl` block,
///     // but it's good for scoping
///     fn on_quit(app: &mut Application)
///     {
///         // ...
///     }
/// }
/// ```
pub trait System
{
    /// insert resources and event handlers from this system
    /// into the application
    fn build(app: &mut Application);
}

/// system function, which does the actual logic of
/// a system
pub type Job = Box<dyn legion::prelude::Schedulable>;

/// enum for an event to systems map, with two possible
/// values:
/// - `unbaked`: systems can still be added and none have been run yet
/// - `backed`: systems can't be added anymore but can be invoked
pub enum Systems
{
    Unbaked
    {
        map: HashMap<TypeId, BinaryHeap<UnbakedSysFn>>
    },
    Baked
    {
        map: HashMap<TypeId, legion::prelude::Schedule>
    }
}

/// represents an unbaked system function
struct UnbakedSysFn
{
    job: Job,

    flush: bool,
    ord: isize,
}

impl Systems
{
    pub(crate) fn new() -> Self
    {
        Systems::Unbaked
        {
            map: Default::default(),
        }
    }

    /// insert a job into this systems collection,
    /// processing ordering and events.
    pub fn insert<T: Event>(&mut self, ord: isize, flush: bool, job: Job)
    {
        if let Systems::Unbaked { map } = self
        {
            map.insert(TypeId::of::<T>(), UnbakedSysFn { job, flush, ord, })
        }
        else
        {
            println!("[warn] attempting to add a system after application start!");
        }
    }
}

impl PartialEq for UnbakedSysFn
{
    fn eq(&self, other: &Self) -> bool
    {
        self.ord.eq(&other.ord)
    }
}

impl PartialOrd for UnbakedSysFn
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering>
    {
        self.ord.partial_cmp(&other.ord)
    }
}

impl Eq for UnbakedSysFn { }

impl Ord for UnbakedSysFn
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering
    {
        self.ord.cmp(&other.ord)
    }
}