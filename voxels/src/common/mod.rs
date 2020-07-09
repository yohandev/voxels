mod consts;

pub use consts::*;

pub mod transform;
pub mod states;
pub mod world;
pub mod block;
pub mod debug;

/// shared system bundle
pub type Bundle =
(
    transform::SLocalToWorld,
    world::SChunkLoad,
    world::SChunkGen,
    debug::SDebugFps,
);