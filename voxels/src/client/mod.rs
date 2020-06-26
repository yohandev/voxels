mod camera;
mod render;

pub use camera::*;
pub use render::*;

// client system bundle
pub type Bundle =
(
    camera::SCameraResize,
    camera::SCameraUniform,
    render::SGraphicsShared,
    render::SGraphicsChunk,
    render::SRender,
);