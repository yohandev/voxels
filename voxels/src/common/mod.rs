mod dir;

pub use dir::*;

pub mod transform;
pub mod debug;
pub mod world;

/// shared system bundle
pub type Bundle =
(
    debug::SDebugFps,
    transform::SLocalToWorld,
);