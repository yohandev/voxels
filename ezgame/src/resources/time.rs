use std::time::*;

#[derive(Debug)]
/// resource created by ezgame to manage frame management and
/// store time information such as delta time. 
pub struct Time
{
    pub(crate) frame: Instant,          // last frame instant
    pub(crate) delta: Duration,         // delta duration between frame and the one before it
}

impl Time
{
    /// time delta between this frame and the one before it
    /// a frame is measured to be consecutive calls of the
    /// event "app_update_event"
    pub fn delta_time(&self) -> &Duration
    {
        &self.delta
    }

    /// 32bit float representation of Time::delta_time(), in
    /// seconds. 
    pub fn delta_time_f32(&self) -> f32
    {
        self.delta.as_secs_f32()
    }

    /// 64bit float representation of Time::delta_time(), in
    /// seconds.
    pub fn delta_time_f64(&self) -> f64
    {
        self.delta.as_secs_f64()
    }

    /// time when the last frame begun
    pub fn last_frame(&self) -> &Instant
    {
        &self.frame
    }

    /// create a new time resource
    pub(crate) fn new() -> Self
    {
        Self
        {
            frame: Instant::now(),
            delta: Duration::default(),
        }
    }
}