// -- declare sub-modules --
mod application;
mod window;
mod input;

// -- export module typdefs --
pub use application::Application;
pub use window::Window;
pub use input::Input;


// -- used by sub-modules --
use winit::event_loop::*;
use winit::event::*;
use crate::ezmath::*;