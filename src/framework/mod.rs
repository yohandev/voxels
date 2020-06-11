// -- declare sub-modules --
mod application;
mod builder;
mod window;
mod input;

// -- export module typdefs --
pub use application::{ Application, State };
pub use builder::ApplicationBuilder;
pub use input::{ Input, KeyCode };
pub use window::Window;


// -- used by sub-modules --
use winit::event_loop::*;
use winit::event::*;
use crate::ezmath::*;