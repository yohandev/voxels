use ezgame::plugins::ezgfx::*;

use crate::resources::gfx::ChunkVertex;

/// chunk mesh component
/// each chunk should have separate geometry,
/// hence why this is a component and not a
/// resource
pub struct ChunkMesh
{
    pub geo: Geometry<ChunkVertex, u32>
}