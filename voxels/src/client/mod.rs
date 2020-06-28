mod camera;
mod render;
mod mesh;

pub use camera::*;
pub use render::*;
pub use mesh::*;

// client system bundle
pub type Bundle =
(
    camera::SCameraResize,
    camera::SCameraUniform,
    render::SGraphicsShared,
    render::SGraphicsChunk,
    render::SRender,
);