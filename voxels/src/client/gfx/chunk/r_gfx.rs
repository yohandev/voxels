use ezgame::gfx::*;
use ezmath::*;

use super::{ ChunkPosBind, ChunkMesh };

/// shared graphic resources for chunks
pub type RGraphicsChunk = Option
<(
    Shader,         // shared vertex shader
    Shader,         // shared fragment shader

    ChunkPosBind,   // shared chunk position uniform
    Pipeline,       // shared rendering pipeline

    ChunkMeshes,    // pool of chunk meshes
)>;

/// resource that stores all the
/// chunk meshes
type ChunkMeshes = std::collections::HashMap<int3, ChunkMesh>;