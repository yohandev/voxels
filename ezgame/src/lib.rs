mod game;
mod app;

pub use game::*;
pub use app::*;

//pub mod window;
//pub mod input;
pub mod time;
pub mod evt;
pub mod ecs;

pub use ::winit;

//#[cfg(feature="plugin-ezgfx")]
//pub mod gfx;

/// system bundle consisting of game systems
#[cfg(not(feature="plugin-ezgfx"))]
pub type GameBundle = (window::SWindow, input::SInput, time::STime);

#[cfg(feature="plugin-ezgfx")]
pub type GameBundle =
(
    //window::SWindow,
    //input::SInput,
    //time::STime,
    //gfx::SGraphicsInit,
    //gfx::SGraphicsResize
);