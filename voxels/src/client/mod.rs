pub mod camera;
pub mod gfx;

// client system bundle
pub type Bundle =
(
    camera::SCameraResize,
    camera::SCameraUniform,
    gfx::SGraphicsShared,
    gfx::SGraphicsChunk,
    gfx::SChunkMesh,
    gfx::SRender,
);