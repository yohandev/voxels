/// event that triggers systems when invoked.
/// representation is a namespace(`str`) + id(`str`).
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Event(&'static str);

/// system execution order, ascending. if system
/// `A::ORDER = 0`, `B::ORDER = 27`, and `C::ORDER
/// = -10`, the systems are executed `C`, then `A`,
/// then `B`.
pub type Order = isize;

/// system execution order
pub mod ord
{
    /// system will have high priority and be
    /// amongst the first to be executed.
    pub const HIGH: super::Order    = -999;
    /// system runs somewhere in the middle of
    /// an event execution.
    pub const MID: super::Order     = 0;
    /// system will have low priority and be
    /// amongst the last to be executed.
    pub const LOW: super::Order     = 999;
}

/// events resource. this is the ezgame provided
/// event system, which recursively pushes and
/// polls events. this uses a concurrent queue
/// internally, so it's recommanded you use a
/// read_resource() for this, to maximize
/// parralelism.
/// ```rust
/// sys("my_system_a")
///     .write_resource::<REvents>()
///     .build(|_, _, events, _|
///     {
///         events.push(MyEvent);
///         // ^^^^ my_system_a blocks the REvents
///         //      resource, can't run in parallel.
///     })
/// sys("my_system_b")
///     .resource_resource::<REvents>()
///     .build(|_, _, events, _|
///     {
///         events.push(MyEvent);
///         // ^^^^ my_system_b can run in parallel
///     })
/// ```
#[derive(Debug, Default)]
pub struct REvents
{
    queue: crossbeam::queue::SegQueue<Event>
}

impl REvents
{
    /// push an event to the event queue. note
    /// that the event won't be immediately
    /// executed, and duplicates can occur.
    pub fn push(&self, e: &Event)
    {
        self.queue.push(*e);
    }

    /// internal: pop the events queue for
    /// processing
    pub(crate) fn pop(&self) -> Option<Event>
    {
        self.queue.pop().ok()
    }

    /// internal: is the events queue empty?
    pub(crate) fn is_empty(&self) -> bool
    {
        self.queue.is_empty()
    }
}

impl Event
{
    /// create a new event(doesn't actually
    /// invoke it)
    pub fn new(name: &'static str) -> Self
    {
        Self(name)
    }
}