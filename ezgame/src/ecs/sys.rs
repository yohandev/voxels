use std::collections::VecDeque;
use std::any::TypeId;

use crate::*;

/// an event-responding system, that operates on entities
/// and their components. it's convention that systems
/// are prefixed with an `S`.
pub trait System: 'static
{
    /// the order in which this system is executed.
    /// if system `B` depends on system `A`, you
    /// should make `B::ORDER` equal to `A::ORDER + 1`.
    fn order() -> isize where Self: Sized { 0 }

    /// should this system be enabled automatically?
    fn auto_start() -> bool where Self: Sized { true }

    /// called exactly once during the later of the
    /// two:
    /// - when the system is created
    /// or
    /// - when the application begins running
    fn on_create(&mut self, app: &mut Application) {}

    /// called whenever the system starts running.
    /// this is called after `on_create` if the system
    /// starts automatically. it's also called when
    /// the system is enabled
    fn on_start_running(&mut self, app: &mut Application) {}

    /// called on the application update, roughly 60
    /// times per second on most machines
    fn on_update(&mut self, app: &mut Application) {}

    /// called whenever the system stops running.
    /// this is called before `on_destroy` if the system
    /// was running before destruction. it's also called
    /// when the system is disabled
    fn on_stop_running(&mut self, app: &mut Application) {}

    /// called exactly once during the earlier of the two:
    /// - when the system is destroyed
    /// or
    /// - when the application quits
    fn on_destroy(&mut self, app: &mut Application) {}
}

/// wrapper around a dyn `System`
pub struct SystemWrapper
{
    inner: Box<dyn System>,
    order: isize,
    enabled: bool,
}

/// collection of systems
#[derive(Default)]
pub struct Systems
{
    list: Vec<SystemWrapper>,
}

/// queue of system commands. each action
/// called on it isn't executed until flushed,
/// to prevent concurrent modification
pub struct SystemsCmdBuf
{
    queue: VecDeque<SystemCmd>,
}

/// queue-able system manipulation commands
enum SystemCmd
{
    /// add a system when flushing
    Add(SystemWrapper),
    /// remove a system when flushing
    Remove(TypeId),
    /// enable a system when flushing
    Enable(TypeId),
    /// disable a system when flushing
    Disable(TypeId),
}

impl SystemWrapper
{
    /// create a new system wrapper from its default
    pub fn new<T: System + Default>() -> Self
    {
        Self
        {
            inner: Box::new(T::default()),

            order: T::order(),
            enabled: T::auto_start(),
        }
    }

    /// create a new system wrapper from an implicit
    /// instance
    pub fn new_val<T: System>(sys: T) -> Self
    {
        Self
        {
            inner: Box::new(sys),

            order: T::order(),
            enabled: T::auto_start(),
        }
    }

    /// is this system enabled?
    pub fn enabled(&self) -> bool
    {
        self.enabled
    }

    /// enables this system, doing nothing if
    /// already enabled
    pub fn enable(&mut self, app: &mut Application)
    {
        if !self.enabled()
        {
            self.inner.on_start_running(app)
        }
        self.enabled = true;
    }

    /// disables this system, doing nothing if
    /// already disabled
    pub fn disable(&mut self, app: &mut Application)
    {
        if self.enabled()
        {
            self.inner.on_stop_running(app)
        }
        self.enabled = false;
    }

    pub(crate) fn create(&mut self, app: &mut Application)
    {
        self.inner.on_create(app);
        if self.enabled()
        {
            self.inner.on_start_running(app);
        }
    }

    pub(crate) fn update(&mut self, app: &mut Application)
    {
        if self.enabled()
        {
            self.inner.on_update(app);
        }
    }

    pub(crate) fn destroy(&mut self, app: &mut Application)
    {
        if self.enabled()
        {
            self.inner.on_stop_running(app);
        }
        self.inner.on_destroy(app);
    }
}

impl Systems
{
    //pub fn create

    pub(crate) fn update(&mut self, app: &mut Application)
    {
        for sys in &mut self.list
        {
            sys.update(app)
        }
    }
}

impl SystemsCmdBuf
{
    /// insert a new system into the application. if a system
    /// of a given type already exists, it's silently overwritten.
    pub fn insert<T: System + Default>(&mut self)
    {
        self.queue
            .push_back(SystemCmd::Add(SystemWrapper::new::<T>()));
    }

    /// insert a new system by value into the application. if a system
    /// of a given type already exists, it's silently overwritten.
    pub fn insert_val<T: System>(&mut self, sys: T)
    {
        self.queue
            .push_back(SystemCmd::Add(SystemWrapper::new_val(sys)));
    }
}