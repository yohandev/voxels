// -- declare sub-modules --
mod application;
mod builder;
mod render;
mod window;
mod input;
mod state;

// -- export module typdefs --
pub use render::{ RenderCtx, Uniform };
pub use builder::ApplicationBuilder;
pub use application::Application;
pub use input::{ Input, KeyCode };
pub use window::Window;
pub use state::State;

// -- used by sub-modules --
use winit::event_loop::*;
use winit::event::*;
use crate::ezmath::*;