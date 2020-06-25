mod game;
mod app;

pub use game::*;
pub use app::*;

pub mod window;
pub mod input;
pub mod time;
pub mod evt;

pub use ::ezgame_ecs as ecs;
pub use ::winit;

#[cfg(feature="plugin-ezgfx")]
pub mod gfx;