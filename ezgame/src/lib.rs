mod game;
mod app;

pub use game::*;
pub use app::*;

pub mod components;
pub mod resources;
pub mod systems;
pub mod events;

pub mod legion { pub use legion::prelude::*; }
pub mod winit { pub use winit::*; }

use crate::legion::*;
use crate::winit::*;