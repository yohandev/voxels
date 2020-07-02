mod consts;

pub use consts::*;

pub mod transform;
pub mod chunk;
pub mod block;
pub mod debug;

/// shared system bundle
pub type Bundle =
(
    transform::SLocalToWorld,
    chunk::SChunkGen,
    debug::SDebugFps,
);