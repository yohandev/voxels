/// resource created by ezgame to queue and poll user and
/// framework invoked events. see the docs on EventsQueue::invoke
/// for specification. 
#[derive(Debug)]
pub struct EventsQueue(Vec<&'static str>);

impl EventsQueue
{
    /// queues an event to invoke on all worlds.
    /// the event will only be invoked *after* all the systems
    /// of the current event have been processed.
    /// # example
    /// if systems Foo, Bar, and Baz are invoked by the "update_event",
    /// in that order, and Bar invokes "my_event", the systems listening
    /// to "my_event" will only run after all Foo, Bar, and Baz are done.
    pub fn invoke(&mut self, event: &'static str)
    {
        self.0.push(event);
    }

    /// create a new Events manager
    pub(crate) fn new() -> Self
    {
        Self(Default::default())
    }

    // get the inner vector contained in this struct
    pub(crate) fn unwrap(self) -> Vec<&'static str>
    {
        self.0
    }
}