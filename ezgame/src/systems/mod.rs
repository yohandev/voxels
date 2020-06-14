use legion::prelude::*;

mod invoke;
mod window;
mod input;

pub(crate) use invoke::system_process_invokes;
pub(crate) use window::system_create_window;
pub        use window::window_system;
pub        use input::input_system;