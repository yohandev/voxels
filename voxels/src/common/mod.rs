mod consts;
mod dir;

pub use consts::*;
pub use dir::*;

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