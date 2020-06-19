use legion::prelude::*;

mod invoke;
mod input;
mod time;

pub(crate) use invoke::system_process_invokes;
pub        use input::input_system;
pub        use time::time_system;