/// event that triggers systems when invoked.
/// internally, it's just a number but the user
/// facing representation is a namespace(`str`) +
/// id(`str`).
pub struct Event(usize);

/// system execution order, ascending. if system
/// `A::ORDER = 0`, `B::ORDER = 27`, and `C::ORDER
/// = -10`, the systems are executed `C`, then `A`,
/// then `B`.
pub type Order = isize;

/// system execution order
pub mod order
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