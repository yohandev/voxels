pub mod camera;
pub mod gfx;

// client system bundle
pub type Bundle =
(
    camera::SFpsController,
    camera::SCameraUniform,
    camera::SCameraResize,
    gfx::SGraphicsShared,
    gfx::SGraphicsChunk,
    gfx::SChunkMesh,
    gfx::SRender,
);