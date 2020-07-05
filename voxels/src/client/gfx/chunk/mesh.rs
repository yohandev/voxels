use ezgame::gfx::*;

use super::{ ChunkPosition, ChunkVertex };

/// the geometry and position uniform of a chunk
pub struct ChunkMesh
{
    pub geo: ChunkGeometry,
    pub pos: ChunkPosBind,
}

/// geometry of a given chunk
pub type ChunkGeometry = Geometry<ChunkVertex, u32>;
/// position bind group of a chunk
pub type ChunkPosBind = BindGroup<(Uniform<ChunkPosition>,)>;

#[derive(Debug, Default)]
/// temporary structure to generate chunk geometry
pub struct ChunkMeshBuilder
{
    pub vert: Vec<ChunkVertex>,
    pub ind: Vec<u32>
}